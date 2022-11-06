use std::net::{IpAddr, SocketAddr, Ipv4Addr, Ipv6Addr};

pub type Config = Vec<Vec<Port>>;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct  Port {
    pub port: u16,
    pub ip: IpAddr,
    pub last_n: Option<usize>,
    pub forget_ms: Option<u64>,
    pub sendto: Option<Vec<SocketAddr>>,
    pub broadcast: Option<bool>,
    pub multicast_addr: Option<Ipv4Addr>,
    pub multicast_ifaddr: Option<Ipv4Addr>,
    pub multicast_ifindex: Option<u32>,
    pub multicast_ttl: Option<u32>,
    pub multicast6_addr: Option<Ipv6Addr>,
    pub multicast6_ifindex: Option<u32>,
    pub ttl: Option<u32>,
    pub sender_period_ms: Option<u64>,
    pub sender_data_base64: Option<String>,
    pub norecv : Option<bool>,
    pub sndbuf: Option<usize>,
    pub rcvbuf: Option<usize>,
    pub write_timeout_ms: Option<u64>,
    pub tos: Option<u32>,
    pub v6only: Option<bool>,
    pub v6_multicast_hops: Option<u32>,
    pub v6_unicast_hops: Option<u32>,
}