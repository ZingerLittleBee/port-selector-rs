mod take_up;

use rand::prelude::*;
use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs, UdpSocket,
};

pub type Port = u16;

#[derive(Copy, Clone, Debug)]
pub struct Selector {
    pub check_tcp: bool,
    pub check_udp: bool,
    pub port_range: (u16, u16),
    pub max_random_times: u16,
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

/// Check whether the port is not used on TCP
pub fn is_free_tcp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_tcp(ipv6).is_some() && test_bind_tcp(ipv4).is_some()
}

/// Check whether the port is not used on UDP
pub fn is_free_udp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_udp(ipv6).is_some() && test_bind_udp(ipv4).is_some()
}

/// Check whether the port is not used on TCP and UDP
pub fn is_free(port: Port) -> bool {
    is_free_tcp(port) && is_free_udp(port)
}

/// The system randomly assigns available TCP ports
pub fn random_free_tcp_port() -> Option<Port> {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

    test_bind_tcp(ipv6).or_else(|| test_bind_tcp(ipv4))
}

/// The system randomly assigns available UDP ports
pub fn random_free_udp_port() -> Option<Port> {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

    test_bind_udp(ipv6).or_else(|| test_bind_udp(ipv4))
}

/// The system randomly assigns available TCP and UDP ports
pub fn random_free_port() -> Option<Port> {
    loop {
        let free_tcp_port = random_free_tcp_port();
        if free_tcp_port.is_some() && is_free_udp(free_tcp_port?) {
            break free_tcp_port;
        }
    }
}

/// Check from `given_port` and return the first available port
/// Return if `given_port` is available; Otherwise `given_port += given_port` until the port is available
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

/// Gets a matching port based on the `Selector` parameter constraint
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

#[cfg(test)]
mod tests {

    use crate::{
        is_free, is_free_tcp, is_free_udp, random_free_port, random_free_tcp_port,
        random_free_udp_port, select_free_port, select_from_given_port,
        take_up::{random_take_up_port, random_take_up_tcp_port, random_take_up_udp_port},
        test_bind_tcp, test_bind_udp, Selector,
    };
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn test_is_free() {
        let used_port = random_take_up_port();
        assert!(!is_free(used_port));
        let free_port = random_free_port();
        assert!(is_free(free_port.unwrap()));
    }

    #[test]
    fn test_is_free_tcp() {
        let used_tcp_port = random_take_up_tcp_port();
        assert!(!is_free_tcp(used_tcp_port));
        let free_port = random_free_tcp_port();
        assert!(is_free_tcp(free_port.unwrap()));
    }

    #[test]
    fn test_is_free_udp() {
        let used_tcp_port = random_take_up_udp_port();
        assert!(!is_free_udp(used_tcp_port));
        let free_port = random_free_udp_port();
        assert!(is_free_udp(free_port.unwrap()));
    }

    #[test]
    fn test_free_tcp_port() {
        let free_tcp_port = random_free_tcp_port();
        assert!(free_tcp_port.is_some());
        assert!(is_free_tcp(free_tcp_port.unwrap()));
    }

    #[test]
    fn test_free_udp_port() {
        let free_udp_port = random_free_udp_port();
        assert!(free_udp_port.is_some());
        assert!(is_free_udp(free_udp_port.unwrap()));
    }

    #[test]
    fn test_pick_unused_port() {
        let used_port = random_take_up_port();
        let selector_fail: Selector = Selector {
            port_range: (used_port, used_port + 1),
            ..Default::default()
        };
        let port = select_free_port(selector_fail);
        println!("selector_fail, port: {:#?}", &port);
        assert!(!port.is_some());

        let selector: Selector = Selector {
            port_range: (50000, 60000),
            ..Default::default()
        };
        for i in 0..100 {
            let port = select_free_port(selector);
            println!("index: {}, port: {:#?}", i, &port.unwrap());
            assert!(&port.unwrap() >= &50000 && &port.unwrap() <= &60000);
            assert!(port.is_some());
        }
    }

    #[test]
    fn test_test_bind_tcp() {
        assert!(test_bind_tcp(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).is_some());
        assert!(test_bind_tcp(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).is_some());
    }

    #[test]
    fn test_test_bind_udp() {
        assert!(test_bind_udp(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).is_some());
        assert!(test_bind_udp(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).is_some());
    }

    #[test]
    fn test_random_free_port() {
        let port = random_free_port().unwrap();
        println!("port: {}", &port);
        assert!(is_free_tcp(port) && is_free_udp(port));
        assert!(random_free_port().is_some());
    }

    #[test]
    fn test_select_from_given_port() {
        let port = select_from_given_port(30000).unwrap();
        println!("port: {}", &port);
        assert!(is_free_tcp(port) && is_free_udp(port));
        let mut used_port = random_take_up_port();
        println!("used_port: {}", &used_port);
        let new_port = select_from_given_port(used_port).unwrap();
        println!("new_port: {}", &new_port);
        let used_port = loop {
            if is_free(used_port) {
                break used_port;
            }
            used_port += 1;
        };
        assert_eq!(new_port, used_port);
    }
}
