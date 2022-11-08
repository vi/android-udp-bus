use std::net::{IpAddr, SocketAddr, Ipv4Addr, Ipv6Addr};

pub type Config = Vec<Vec<Port>>;

#[derive(serde::Deserialize)]
#[cfg_attr(feature="schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)] 
/// Configuration of a specific bound UDP port in a hub. Data received on any port of the hub would be transferred
/// to all other clients (on that port and on other ports of the hub) of the hub, except of sender.
/// Config file is array of array of these objects, i.e. collection of hubs, each having a collection of ports.
pub struct  Port {
    /// Port to bind this port to. From 1 to 65535
    pub port: u16,
    /// IP address to bind this port to. Can be IPv4 or IPv6.
    pub ip: IpAddr,
    /// Number of recently seen (recvfrom-ed) peers to remember, besides the `sendto` list which are permanently kept track of.
    /// If 0, no peers are added to the list (but their incoming packets are still forwarded to the hub, unless `norecv` is set).
    /// Datagrams seen by the hub are sent to the peers in the list. Disovered peers are marked with `T` (transient) in stats.
    pub last_n: Option<usize>,
    /// If set, forget discovered peers after this amount of milliseconds. If unset, they are remembered forever, or
    /// until some newer peer evicts the most long unseen peer.
    pub forget_ms: Option<u64>,
    /// List of permanent (`P`) socket addresses to send packets to.
    pub sendto: Option<Vec<SocketAddr>>,
    /// Set `SO_BROADCAST` to this value
    pub broadcast: Option<bool>,
    /// Join this IPv4 multicast group. Requires specification of interface,
    /// either by IP (`multicast_ifaddr`) or by index (`multicast_ifindex`).
    pub multicast_addr: Option<Ipv4Addr>,
    /// IP of the network interface to join multicast
    pub multicast_ifaddr: Option<Ipv4Addr>,
    /// Index of the network interface to join multicast
    pub multicast_ifindex: Option<u32>,
    /// Set specific TTL value for the outgoing IPv4 multicast datagrams (IP_MULTICAST_TTL), instead of usual 1.
    pub multicast_ttl: Option<u32>,
    /// Join this IPv6 multicast group. Requires `multicast6_ifindex`.
    pub multicast6_addr: Option<Ipv6Addr>,
    pub multicast6_ifindex: Option<u32>,
    /// Set IP_TTL
    pub ttl: Option<u32>,
    /// Periodically send specified data buffer to addresses specified in `sendto` field (not to the hub itself).
    /// By default it sends a zero-length datagram
    pub sender_period_ms: Option<u64>,
    /// Content of the datagram for `sender_period_ms`, base64-encoded.
    pub sender_data_base64: Option<String>,
    /// Don't call `recvmsg` on this port. Renders `last_n` useless.
    pub norecv : Option<bool>,
    /// Set socket send buffer size (SO_SNDBUF).
    pub sndbuf: Option<usize>,
    /// Set socket receive buffer size (SO_RCVBUF).
    pub rcvbuf: Option<usize>,
    /// Set SO_SNDTIMEO
    pub write_timeout_ms: Option<u64>,
    /// Set IP_TOS. There is currently no IPv4 analogue.
    pub tos: Option<u32>,
    /// Set IPV6_V6ONLY before binding the socket 
    pub v6only: Option<bool>,
    /// Set IPV6_MULTICAST_HOPS
    pub v6_multicast_hops: Option<u32>,
    /// Set IPV6_UNICAST_HOPS
    pub v6_unicast_hops: Option<u32>,
    /// Queue length of this hub (not a port-specific option). Default is 16.
    /// This for internal in-app queue, not qlen of the network interface.
    /// Can be set only on one port of the hub (or set to the same value for all ports).
    /// Too small value may lead to dropped (`lagged`) datagrams, too large value may lead to buffer bloat.
    pub qlen: Option<usize>,
}
