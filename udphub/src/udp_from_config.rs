
use std::{net::SocketAddr, time::Duration};

use socket2::{Domain, InterfaceIndexOrAddress, SockAddr, Type};
use tokio::net::UdpSocket;

pub fn udp_from_config(c: &crate::config::Port) -> anyhow::Result<UdpSocket> {
    let sa = SocketAddr::new(c.ip, c.port);
    let s = socket2::Socket::new(Domain::for_address(sa), Type::DGRAM, None)?;
    let _ = s.set_reuse_address(true);
    let _ = s.set_reuse_port(true);

    if let Some(x) = &c.v6only {
        s.set_only_v6(*x)?;
    }

    let sa = SockAddr::from(sa);
    s.bind(&sa)?;
    s.set_nonblocking(true)?;

    if let Some(x) = &c.broadcast {
        s.set_broadcast(*x)?
    }
    if let (Some(x), Some(y)) = (&c.multicast6_addr, &c.multicast6_ifindex) {
        s.join_multicast_v6(x, *y)?;
    }
    if let (Some(x), Some(y)) = (&c.multicast_addr, &c.multicast_ifindex) {
        s.join_multicast_v4_n(x, &InterfaceIndexOrAddress::Index(*y))?;
    }
    if let (Some(x), Some(y)) = (&c.multicast_addr, &c.multicast_ifaddr) {
        s.join_multicast_v4(x, y)?;
    }
    if let Some(x) = &c.multicast_ttl {
        s.set_multicast_ttl_v4(*x)?;
    }
    if let Some(x) = &c.ttl {
        s.set_ttl(*x)?;
    }
    if let Some(x) = &c.tos {
        s.set_tos(*x)?;
    }
    if let Some(x) = &c.v6_unicast_hops {
        s.set_unicast_hops_v6(*x)?;
    }
    if let Some(x) = &c.v6_multicast_hops {
        s.set_multicast_hops_v6(*x)?;
    }
    if let Some(x) = &c.write_timeout_ms {
        s.set_write_timeout(Some(Duration::from_millis(*x)))?;
    }
    if let Some(x) = &c.sndbuf {
        s.set_send_buffer_size(*x)?;
    }
    if let Some(x) = &c.rcvbuf {
        s.set_recv_buffer_size(*x)?;
    }

    let s = UdpSocket::from_std(s.into())?;
    Ok(s)
}
