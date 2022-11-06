use std::{sync::atomic::Ordering::Relaxed, time::Duration};

use tokio::{sync::{oneshot, mpsc}, time::Instant};
use tokio_util::sync::{CancellationToken, DropGuard};
use humansize::{format_size, BINARY};

use crate::{
    config::Config,
    hub::{self, Hub},
};

pub struct App {
    inner: Option<AppImpl>,
    pub error: Option<String>,
}

type GetStatsCall = (GetStatsMode,oneshot::Sender<String>);

struct AppImpl {
    _quit: DropGuard,
    get_stats: mpsc::Sender<GetStatsCall>
}

pub enum GetStatsMode {
    Short,
    Long,
}

impl App {
    pub fn new() -> Self {
        App { inner: None, error: None }
    }
    pub fn start(&mut self, config: &str) {
        let c: Config = match serde_json::from_str(config) {
            Ok(c) => c,
            Err(e) => {
                self.error = Some(format!("config error: {}", e));
                return;
            }
        };
        match AppImpl::start(c) {
            Ok(i) => self.inner = Some(i),
            Err(e) => {
                self.error = Some(format!("Error: {}", e));
            }
        }
    }

    pub fn get_stats(&self, mode: GetStatsMode) -> Option<String> {
        if let Some(i) = &self.inner {
            let (tx,rx) = oneshot::channel();
            match i.get_stats.try_send((mode,tx)) {
                Ok(()) => (),
                Err(_) => return None,
            };
            match rx.blocking_recv() {
                Ok(x) => Some(x),
                Err(_) => None
            }
        } else {
            None
        }
    }
}

impl AppImpl {
    pub fn start(config: Config) -> anyhow::Result<AppImpl> {
      
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()?;
        let rt_guard = rt.enter();

        let quit = CancellationToken::new();

        let mut hubs : Vec<Hub> = Vec::with_capacity(config.len());

        let cancel_guard = quit.clone().drop_guard();

        for hub_config in config {
            hubs.push(hub::Hub::start(&hub_config, quit.clone(), &rt)?);
        }

        let (get_stats, mut get_stats_rx) = mpsc::channel::<GetStatsCall>(1);

        drop(rt_guard);
        std::thread::spawn(move || {
            let rt = rt;
            let hubs = hubs;
            rt.block_on(async move {
                while let Some((mode,get_stats_slot)) = get_stats_rx.recv().await {
                    let ret = match mode {
                        GetStatsMode::Short => {
                            let mut total_rx_dgrams = 0;
                            let mut total_rx_bytes = 0;
                            for h in &hubs {
                                for (_, pq) in &h.port_stats_queriers[..] {
                                    let (tx,rx) = oneshot::channel();
                                    let _ = pq.send((GetStatsMode::Short, tx)).await;
                                    if let Ok(ps) = rx.await {
                                        total_rx_bytes += ps.port_wide.recv_bytes;
                                        total_rx_dgrams += ps.port_wide.recv_dgrams;
                                    }
                                }
                            }
                            format!("{}, {} dgrams", format_size(total_rx_bytes, BINARY), total_rx_dgrams)
                        }
                        GetStatsMode::Long => {
                            let mut buf = String::with_capacity(1024);
                            let now = Instant::now();
                            let ta = timeago::Formatter::new();

                            let mut kbps_formater = humansize::FormatSizeOptions::default();
                            kbps_formater.base_unit = humansize::BaseUnit::Bit;
                            kbps_formater.kilo = humansize::Kilo::Binary;
                            kbps_formater.units = humansize::Kilo::Binary;
                            kbps_formater.decimal_places=1;
                            kbps_formater.decimal_zeroes=1;
                            kbps_formater.long_units=false;
                            kbps_formater.space_after_value=true;
                            kbps_formater.suffix="/s";

                            let mut bytes_formatter = humansize::FormatSizeOptions::default();
                            bytes_formatter.base_unit = humansize::BaseUnit::Byte;
                            bytes_formatter.kilo = humansize::Kilo::Binary;
                            bytes_formatter.units = humansize::Kilo::Binary;
                            bytes_formatter.decimal_places=2;
                            bytes_formatter.decimal_zeroes=2;
                            bytes_formatter.long_units=false;
                            bytes_formatter.space_after_value=true;

                            if hubs.is_empty() {
                                buf.push_str("No hubs\n");
                            } 
                            for (hub_n, h) in hubs.iter().enumerate() {
                                if hubs.len() != 1 {
                                    buf.push_str(&format!("* Hub {} *\n", hub_n+1));
                                }
                                if h.port_stats_queriers.is_empty() {
                                    buf.push_str("No ports\n");
                                } 
                                for (portaddr, pq) in &h.port_stats_queriers[..] {
                                    buf.push_str(&format!("port {}\n", portaddr));
                                    
                                    let (tx,rx) = oneshot::channel();
                                    let _ = pq.send((GetStatsMode::Long, tx)).await;
                                    if let Ok(mut ps) = rx.await {
                                        let w = ps.port_wide;
                                        buf.push_str(&format!("  recv {} ({} dgrams)\n", format_size(w.recv_bytes, &bytes_formatter), w.recv_dgrams));
                                        buf.push_str(&format!("  sent {} ({} dgrams)\n", format_size(w.send_bytes, &bytes_formatter), w.send_dgrams));
                                        if w.lagged > 0 || w.send_errors > 0 || w.recv_errors > 0 {
                                            buf.push_str(&format!("  lagged {} senderrs {} recverrs {}\n", w.lagged, w.send_errors, w.recv_errors));
                                        }
                                        if w.dgrams_to_nowhere > 0 {
                                            buf.push_str(&format!("  datagrams to nowhere {}\n", w.dgrams_to_nowhere));
                                        }
                                        ps.peers.sort_by_key(|x|(x.1,x.0));
                                        for (pa, pt, ps) in &ps.peers[..] {
                                            let pt = match pt {
                                                hub::SockAddrType::Permanent => 'P',
                                                hub::SockAddrType::Transient => 'T',
                                            };
                                            buf.push_str(&format!("  peer {} {}\n", pt, pa));
                                            let joined_ago = now.duration_since(ps.begin);
                                            let recv_ago = now.duration_since(ps.last_recv); 
                                            buf.push_str(&format!("    joined {}\n    last seen {}\n",ta.convert(joined_ago), ta.convert(recv_ago)));
                                            buf.push_str(&format!("    recv {} ({} dgrams)", format_size(ps.recv_bytes, &bytes_formatter), ps.recv_dgrams));
                                            
                                            let tm = now.duration_since(ps.saved_recv_counter_ts1);
                                            if tm > Duration::from_secs(5) {
                                                let nb = ps.recv_bytes.saturating_sub(ps.saved_recv_counter_value1);
                                                let bps = nb as f64 * 8.0 / tm.as_secs_f64();
                                                struct Q(f64);
                                                // https://github.com/LeopoldArkham/humansize/issues/27
                                                impl humansize::ToF64 for Q {
                                                    fn to_f64(&self) -> f64 {
                                                        self.0
                                                    }
                                                }
                                                impl humansize::Unsigned for Q {}
                                                let bps = Q(bps);
                                                buf.push_str(&format!(" {}", format_size(bps, &kbps_formater)));
                                            }
                                            buf.push_str("\n");
                                            
                                            let (b, d) = (ps.send_bytes.load(Relaxed),ps.send_dgrams.load(Relaxed));
                                            buf.push_str(&format!("    sent {} ({} dgrams)\n", format_size(b, &bytes_formatter), d));
                                        }
                                    }
                                }
                            }
                            buf
                        }
                    };
                    let _ = get_stats_slot.send(ret);
                }
            })
        });

        Ok(AppImpl {
            _quit: cancel_guard,
            get_stats,
        })
    }
}
