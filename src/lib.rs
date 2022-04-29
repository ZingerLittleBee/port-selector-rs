use rand::prelude::*;
use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs, UdpSocket,
};

pub type Port = u16;

#[derive(Copy, Clone)]
pub struct Selector {
    check_tcp: bool,
    check_udp: bool,
    port_range: (u16, u16),
    max_random_times: u16,
}

impl Default for Selector {
    fn default() -> Self {
        Selector {
            check_tcp: true,
            check_udp: true,
            port_range: (0, 65535),
            max_random_times: 100,
        }
    }
}

// Try to bind to a socket using TCP
fn test_bind_tcp<A: ToSocketAddrs>(addr: A) -> Option<Port> {
    Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
}

// Try to bind to a socket using UDP
fn test_bind_udp<A: ToSocketAddrs>(addr: A) -> Option<Port> {
    Some(UdpSocket::bind(addr).ok()?.local_addr().ok()?.port())
}

/// Check if a port is free on TCP
pub fn is_free_tcp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_tcp(ipv6).is_some() && test_bind_tcp(ipv4).is_some()
}

pub fn is_free_udp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_udp(ipv6).is_some() && test_bind_udp(ipv4).is_some()
}

/// Check if a port is free on both TCP and UDP
pub fn is_free(port: Port) -> bool {
    is_free_tcp(port) && is_free_udp(port)
}

/// Asks the OS for a free tcp port
pub fn random_free_tcp_port() -> Option<Port> {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

    test_bind_tcp(ipv6).or_else(|| test_bind_tcp(ipv4))
}

// Asks the OS for a free udp port
pub fn random_free_udp_port() -> Option<Port> {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

    test_bind_udp(ipv6).or_else(|| test_bind_udp(ipv4))
}

// Asks the OS for a free port
pub fn random_free_port() -> Option<Port> {
    loop {
        let free_tcp_port = random_free_tcp_port();
        if free_tcp_port.map(|f| is_free_udp(f))? {
            break free_tcp_port;
        }
    }
}

// Select free port from given port, if not given_port++
pub fn select_from_given_port(given_port: Port) -> Option<Port> {
    let mut port = given_port;
    loop {
        if is_free(port) {
            break Some(port);
        } else {
            port += 1;
        }
    }
}

// Select an available port that is meet the conditions
pub fn select_free_port(selector: Selector) -> Option<Port> {
    let mut rng = rand::thread_rng();
    let (from, to) = selector.port_range;
    if selector.check_tcp && selector.check_udp {
        for _ in 0..selector.max_random_times {
            let port = rng.gen_range(from..to);
            if is_free(port) {
                return Some(port);
            }
        }
    } else if selector.check_tcp {
        for _ in 0..selector.max_random_times {
            let port = rng.gen_range(from..to);
            if is_free_tcp(port) {
                return Some(port);
            }
        }
    } else if selector.check_udp {
        for _ in 0..selector.max_random_times {
            let port = rng.gen_range(from..to);
            if is_free_udp(port) {
                return Some(port);
            }
        }
    }
    // Give up
    None
}
