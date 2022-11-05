use std::{net::SocketAddr, sync::atomic::AtomicU64, time::Duration};

use bytes::Bytes;
use lru_time_cache::LruCache;
use tokio::{net::UdpSocket, sync::{broadcast::{error::RecvError, self}, oneshot, mpsc}, time::Instant, runtime::Runtime};
use tokio_util::sync::CancellationToken;

use std::sync::atomic::Ordering::Relaxed;

use crate::{
    config::{self},
    udp_from_config::udp_from_config, app::GetStatsMode,
};

type DgramSender = broadcast::Sender<(Bytes, SocketAddr)>;
type DgramReceiver = broadcast::Receiver<(Bytes, SocketAddr)>;
type StatsRequestReceiver = mpsc::Receiver<(GetStatsMode, oneshot::Sender<ReportStats>)>;
type StatsRequestSender = mpsc::Sender<(GetStatsMode, oneshot::Sender<ReportStats>)>;

pub struct Hub {
    pub port_stats_queriers : Box<[(SocketAddr, StatsRequestSender)]>,
}

pub struct PortTask {
    socket: UdpSocket,
    rx: DgramReceiver,
    tx: DgramSender,
    lru: Option<LruCache<SocketAddr, EntryStats>>,
    quit: CancellationToken,
    send_addrs: Option<Box<[(SocketAddr, EntryStats)]>>,
    stats_requests: StatsRequestReceiver,
}

impl Hub {
    pub fn start(c: &[config::Port], quit: CancellationToken, rt: &Runtime) -> anyhow::Result<Hub> {
        // let mut sockets = Vec::with_capacity(c.len());
        let mut port_stats_queriers = Vec::with_capacity(c.len());
        let (tx, _rx) = broadcast::channel(2);
        for sc in c {
            let sa = SocketAddr::new(sc.ip, sc.port);
            let s = udp_from_config(sc)?;

            let lru = match sc.last_n {
                Some(0) => None,
                None => None,
                Some(x) => Some(if let Some(ms) = sc.forget_ms {
                    LruCache::with_expiry_duration_and_capacity(Duration::from_millis(ms), x)
                } else {
                    LruCache::with_capacity(x)
                }),
            };
            let send_addrs = sc.sendto.as_ref().map(|aa| {
                aa.iter()
                    .map(|a| (*a, EntryStats::default()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            });
            let (stats_requests_tx, stats_requests) = mpsc::channel(1);
            port_stats_queriers.push((sa, stats_requests_tx));
            let task = PortTask {
                socket: s,
                lru,
                rx: tx.subscribe(),
                tx: tx.clone(),
                quit: quit.clone(),
                send_addrs,
                stats_requests,
            };
            rt.spawn(task.start());
        }
        Ok(Hub {
            port_stats_queriers : port_stats_queriers.into_boxed_slice(),
        })
    }
}

#[derive(Clone, Copy, Default)]
pub struct PortStats {
    pub recv_errors: u64,
    pub dgrams_to_nowhere: u64,
    pub send_errors: u64,
    pub lagged: u64,
    pub recv_dgrams: u64,
    pub recv_bytes: u64,
    pub send_dgrams: u64,
    pub send_bytes: u64,
}

pub struct EntryStats {
    pub recv_dgrams: u64,
    pub recv_bytes: u64,
    pub send_dgrams: AtomicU64,
    pub send_bytes: AtomicU64,
    pub begin: Instant,
    pub last_recv: Instant,
    pub saved_recv_counter_ts1: Instant,
    pub saved_recv_counter_value1: u64,
    pub saved_recv_counter_ts2: Instant,
    pub saved_recv_counter_value2: u64,
}

impl Default for EntryStats {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            recv_dgrams: 0,
            recv_bytes: 0,
            send_dgrams: AtomicU64::new(0),
            send_bytes: AtomicU64::new(0),
            begin: now,
            last_recv: now,
            saved_recv_counter_ts1: now,
            saved_recv_counter_value1: 0,
            saved_recv_counter_ts2: now,
            saved_recv_counter_value2: 0,
        }
    }
}

impl Clone for EntryStats {
    fn clone(&self) -> Self {
        Self {
            recv_dgrams: self.recv_dgrams.clone(),
            recv_bytes: self.recv_bytes.clone(),
            send_dgrams: AtomicU64::new(self.send_dgrams.load(Relaxed)),
            send_bytes: AtomicU64::new(self.send_bytes.load(Relaxed)),
            begin: self.begin.clone(),
            last_recv: self.last_recv.clone(),
            saved_recv_counter_ts1: self.saved_recv_counter_ts1,
            saved_recv_counter_value1: self.saved_recv_counter_value1,
            saved_recv_counter_ts2: self.saved_recv_counter_ts2,
            saved_recv_counter_value2: self.saved_recv_counter_value2,
        }
    }
}

impl EntryStats {
    fn datagram_received(&mut self, n: usize, now: Instant) {
        let n = n as u64;
        self.recv_bytes += n;
        self.recv_dgrams += 1;
        self.last_recv = now;

        if now.duration_since(self.saved_recv_counter_ts2) > Duration::from_secs(10) {
            self.saved_recv_counter_value1 = self.saved_recv_counter_value2;
            self.saved_recv_counter_ts1 = self.saved_recv_counter_ts2;
            self.saved_recv_counter_value2 = self.recv_bytes;
            self.saved_recv_counter_ts2 = now;
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum SockAddrType {
    Permanent,
    Transient,
}

pub struct ReportStats {
    pub port_wide: PortStats,
    pub peers: Box<[(SocketAddr, SockAddrType, EntryStats)]>,
}

impl PortTask {
    async fn start(mut self) {
        let mut buf = bytes::BytesMut::with_capacity(65536);
        buf.resize(2048, 0);

        let mut stats = PortStats::default();
        loop {
            tokio::select! {
                _cancelled = self.quit.cancelled() => {
                    break;
                }
                ret = self.socket.recv_from(&mut buf[..]) => {
                    match ret {
                        Ok((n, from)) => {
                            let now = Instant::now();
                            stats.recv_bytes+=n as u64;
                            stats.recv_dgrams+=1;
                            buf.truncate(n);
                            let b = buf.split().freeze();
                            if buf.capacity() < 2048 {
                                buf.reserve(65536);
                            }
                            buf.resize(2048, 0);
                            match self.tx.send((b, from)) {
                                Ok(_) => (),
                                Err(_) => stats.dgrams_to_nowhere+=1,
                            }
                            let mut already_handled = false;
                            if let Some(ref mut saa) = self.send_addrs {
                                for (sa, es) in &mut saa[..] {
                                    if *sa == from {
                                        already_handled=true;

                                        es.datagram_received(n, now);
                                    }
                                }
                            }
                            if !already_handled {
                                if let Some(ref mut c) = self.lru {
                                    match c.entry(from) {
                                        lru_time_cache::Entry::Vacant(ve) => {
                                            let es = EntryStats {
                                                begin: now,
                                                last_recv: now,
                                                recv_bytes: n as u64,
                                                recv_dgrams: 1,
                                                send_bytes: AtomicU64::new(0),
                                                send_dgrams: AtomicU64::new(0),
                                                saved_recv_counter_ts1: now,
                                                saved_recv_counter_value1: n as u64,
                                                saved_recv_counter_ts2: now,
                                                saved_recv_counter_value2: n as u64,
                                            };
                                            ve.insert(es);
                                        }
                                        lru_time_cache::Entry::Occupied(oe) => {
                                            let es = oe.into_mut();
                                            es.datagram_received(n, now);
                                        }
                                    }
                                }
                            }
                        }
                        Err(_e) => {
                            stats.recv_errors+=1;
                        }
                    }
                }
                ret = self.rx.recv() => {
                    match ret {
                        Err(RecvError::Closed) => {
                            // ?
                        }
                        Err(RecvError::Lagged(x)) => {
                            stats.lagged += x;
                        }
                        Ok((b, from_addr)) => {
                            if let Some(addrs) = &mut self.send_addrs {
                                for (sa,es) in &mut addrs[..] {
                                    if sa != &from_addr {
                                        match self.socket.send_to(&b[..], *sa).await {
                                            Ok(m) => {
                                                stats.send_bytes+=m as u64;
                                                stats.send_dgrams+=1;
                                                es.send_bytes.fetch_add(m as u64, Relaxed);
                                                es.send_dgrams.fetch_add(1, Relaxed);
                                            }
                                            Err(_) => {
                                                stats.send_errors+=1;
                                            }
                                        }
                                    }
                                }
                            }
                            if let Some(c) = &mut self.lru {
                                for (sa, es) in c.peek_iter() {
                                    if sa != &from_addr {
                                        match self.socket.send_to(&b[..], *sa).await {
                                            Ok(m) => {
                                                stats.send_bytes+=m as u64;
                                                stats.send_dgrams+=1;
                                                es.send_bytes.fetch_add(m as u64, Relaxed);
                                                es.send_dgrams.fetch_add(1, Relaxed);
                                            }
                                            Err(_) => {
                                                stats.send_errors+=1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                stats_request = self.stats_requests.recv() => {
                    if let Some((mode, stats_tx)) = stats_request {
                        let mut cap = 0;
                        if matches!(mode, GetStatsMode::Long) {
                            if let Some(x) = &self.send_addrs {
                                cap += x.len();
                            }
                            if let Some(x) = &self.lru {
                                cap += x.len();
                            }
                        }
                        let mut peers : Vec<(SocketAddr, SockAddrType, EntryStats)> = Vec::with_capacity(cap);

                        if matches!(mode, GetStatsMode::Long) {
                            if let Some(x) = &self.send_addrs {
                                peers.extend(x.iter().map(|x|(x.0, SockAddrType::Permanent, x.1.clone())));
                            }
                            if let Some(x) = &self.lru {
                                peers.extend(x.peek_iter().map(|x|(*x.0, SockAddrType::Transient, x.1.clone())));
                            }
                        }
                        
                        let _ = stats_tx.send(ReportStats {
                            peers : peers.into_boxed_slice(),
                            port_wide: stats.clone(),
                        });
                    }
                }
            }
        }
    }
}
