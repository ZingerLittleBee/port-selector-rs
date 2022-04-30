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
        if free_tcp_port.map(|f| is_free_udp(f))? {
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
        random_free_udp_port, select_free_port, select_from_given_port, test_bind_tcp,
        test_bind_udp, Selector,
    };
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, UdpSocket};

    #[test]
    fn test_is_free() {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let port = listener.local_addr().ok().unwrap().port();
        assert_eq!(is_free(port), false);
        assert_eq!(is_free_tcp(port), false);
        assert_eq!(is_free_udp(port), true);
    }

    #[test]
    fn test_is_free_tcp() {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let port = listener.local_addr().ok().unwrap().port();
        assert_eq!(is_free(port), false);
        assert_eq!(is_free_tcp(port), false);
    }

    #[test]
    fn test_is_free_udp() {
        let listener = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let port = listener.local_addr().ok().unwrap().port();
        assert_eq!(is_free(port), false);
        assert_eq!(is_free_udp(port), false);
    }

    #[test]
    fn test_ask_free_tcp_port() {
        assert_eq!(random_free_tcp_port().is_some(), true);
    }

    #[test]
    fn test_ask_free_udp_port() {
        assert_eq!(random_free_udp_port().is_some(), true);
    }

    #[test]
    fn test_pick_unused_port() {
        let listener = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let port = listener.local_addr().ok().unwrap().port();
        let selector_fail: Selector = Selector {
            port_range: (port, port + 1),
            ..Default::default()
        };
        let port = select_free_port(selector_fail);
        println!("selector_fail, port: {:#?}", &port);
        assert_eq!(port.is_some(), false);

        let selector: Selector = Selector {
            port_range: (50000, 60000),
            ..Default::default()
        };
        for i in 0..100 {
            let port = select_free_port(selector);
            println!("index: {}, port: {:#?}", i, &port.unwrap());
            assert_eq!(&port.unwrap() >= &50000 && &port.unwrap() <= &60000, true);
            assert_eq!(port.is_some(), true);
        }
    }

    #[test]
    fn test_test_bind_tcp() {
        assert_eq!(
            test_bind_tcp(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).is_some(),
            true
        );
    }

    #[test]
    fn test_test_bind_udp() {
        assert_eq!(
            test_bind_udp(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).is_some(),
            true
        );
    }

    #[test]
    fn test_random_free_port() {
        let port = random_free_port().unwrap();
        println!("port: {}", &port);
        assert_eq!(is_free_tcp(port) && is_free_udp(port), true);
        assert_eq!(random_free_port().is_some(), true);
    }

    #[test]
    fn test_select_from_given_port() {
        let port = select_from_given_port(30000).unwrap();
        println!("port: {}", &port);
        assert_eq!(is_free_tcp(port) && is_free_udp(port), true);
        let listener = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).unwrap();
        let used_port = listener.local_addr().ok().unwrap().port();
        println!("used_port: {}", &used_port);
        let new_port = select_from_given_port(used_port).unwrap();
        println!("new_port: {}", &new_port);
        assert_eq!(new_port, used_port + 1);
    }
}
