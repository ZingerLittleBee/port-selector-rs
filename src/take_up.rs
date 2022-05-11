use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener, UdpSocket},
    thread,
};

use crate::{is_free, is_free_tcp, is_free_udp, random_free_port, Port};

/// Run TCP Server to take up port on TCP
fn set_up_tcp_listener(port: Option<Port>) -> Port {
    let port = port.unwrap_or_else(|| 0);
    if port > 0 && !is_free_tcp(port) {
        return port;
    }
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).unwrap();
    let port = listener.local_addr().ok().unwrap().port();
    thread::spawn(move || {
        listener.accept().expect("Failed to accept TCP connection");
    });
    port
}

/// Take up given port on TCP
pub fn take_up_tcp_port(port: Port) -> bool {
    !is_free_tcp(set_up_tcp_listener(Some(port)))
}

/// Take up port randomly on TCP
pub fn random_take_up_tcp_port() -> Port {
    set_up_tcp_listener(None)
}

/// Run UDP Server to take up port on UDP
fn set_up_udp_listener(port: Option<Port>) -> Port {
    let port = port.unwrap_or_else(|| 0);
    if port > 0 && !is_free_udp(port) {
        return port;
    }
    let listener = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).unwrap();
    let port = listener.local_addr().ok().unwrap().port();
    thread::spawn(move || {
        listener
            .recv_from(&mut [0u8])
            .expect("Failed to receive UDP packet");
    });
    port
}

/// Take up given port on UDP
pub fn take_up_udp_port(port: Port) -> bool {
    !is_free_udp(set_up_udp_listener(Some(port)))
}

/// Take up port randomly on UDP
pub fn random_take_up_udp_port() -> Port {
    set_up_udp_listener(None)
}

/// Take up port randomly on TCP && UDP
pub fn take_up_port(port: Port) -> bool {
    if is_free_tcp(port) {
        take_up_tcp_port(port);
    }
    if is_free_udp(port) {
        take_up_udp_port(port);
    }
    return !is_free(port);
}

/// Take up port randomly on TCP && UDP
pub fn random_take_up_port() -> Port {
    loop {
        let free_port = random_free_port().expect("Fail to get free port");
        if take_up_port(free_port) {
            break free_port;
        }
    }
}

#[cfg(test)]
mod take_up_tests {
    use crate::{
        is_free, is_free_tcp, is_free_udp, random_free_port, random_free_tcp_port,
        random_free_udp_port,
        take_up::{
            random_take_up_port, random_take_up_tcp_port, random_take_up_udp_port,
            set_up_udp_listener, take_up_port, take_up_tcp_port, take_up_udp_port,
        },
    };

    use super::set_up_tcp_listener;

    #[test]
    fn test_set_up_tcp_listener() {
        let free_tcp_port = random_free_tcp_port();
        assert_eq!(free_tcp_port.unwrap(), set_up_tcp_listener(free_tcp_port));
        assert!(!is_free_tcp(free_tcp_port.unwrap()));
    }

    #[test]
    fn test_set_up_udp_listener() {
        let free_udp_port = random_free_udp_port();
        assert_eq!(free_udp_port.unwrap(), set_up_udp_listener(free_udp_port));
        assert!(!is_free_udp(free_udp_port.unwrap()));
    }

    #[test]
    fn test_take_up_tcp_port() {
        let free_tcp_port = random_free_tcp_port();
        assert!(free_tcp_port.is_some());
        let res = take_up_tcp_port(free_tcp_port.unwrap());
        assert_eq!(res, !is_free_tcp(free_tcp_port.unwrap()));
    }

    #[test]
    fn test_random_take_up_tcp_port() {
        let take_up_tcp_port = random_take_up_tcp_port();
        assert_eq!(is_free_tcp(take_up_tcp_port), false);
    }

    #[test]
    fn test_take_up_udp_port() {
        let free_udp_port = random_free_udp_port();
        assert!(free_udp_port.is_some());
        assert_eq!(
            take_up_udp_port(free_udp_port.unwrap()),
            !is_free_udp(free_udp_port.unwrap())
        );
    }

    #[test]
    fn test_random_take_up_udp_port() {
        let take_up_udp_port = random_take_up_udp_port();
        assert!(!is_free_udp(take_up_udp_port));
    }

    #[test]
    fn test_take_up_port() {
        let free_port = random_free_port();
        assert!(free_port.is_some());
        let is_take_up_port = take_up_port(free_port.unwrap());
        assert!(is_take_up_port);
        assert_eq!(is_free(free_port.unwrap()), false);
    }

    #[test]
    fn test_random_take_up_port() {
        let take_up_port = random_take_up_port();
        assert!(!is_free(take_up_port));
        assert!(!is_free_tcp(take_up_port));
        assert!(!is_free_udp(take_up_port));
    }
}
