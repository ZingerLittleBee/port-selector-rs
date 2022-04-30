Language : 🇺🇸 English | [🇨🇳 简体中文](./README.zh-CN.md)

<p align="center"><font size="7px">port-selector</font></p>
<p align="center">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/port-selector" />
    <img alt="Crates.io" src="https://img.shields.io/crates/d/port-selector" />
    <img alt="Last Commit" src="https://img.shields.io/github/last-commit/ZingerLittleBee/port-selector-rs" />
</p>
<p align="center">
    <a alt="docs.rs" src="https://docs.rs/port-selector/0.1.1/port_selector/"><img src="https://img.shields.io/docsrs/port-selector" /></a>
    <a src="https://github.com/ZingerLittleBee/port-selector-rs/actions" alt="GitHub Actions CI">
        <img src="https://img.shields.io/github/workflow/status/ZingerLittleBee/port-selector-rs/Test%20CI" /></a>
    <a src="./LICENSE">
    <img alt="Crates.io" src="https://img.shields.io/crates/l/port-selector" /></a>
</p>

## Overview

port-selector is a rust library that mainly provides port availability checking and port filtering based on conditions.

## Install
1. Get the latest version -> https://crates.io/crates/port-selector

2. Add the dependent
```toml
[dependencies]
port-selector = "x.x.x"
```

3. use
```rust
use port_selector::{is_free, Port};

fn main() {
    let check_port: Port = 3000;
    println!("is_free({}) = {}", check_port, is_free(check_port));
}
```

## Goods
type -> [Port](#port) · [Selector](#selector) 

fn -> [is_free_tcp](#isfreetcp) · [is_free_udp](#isfreeudp) · [is_free](#isfree) · [random_free_tcp_port](#randomfreetcpport) · [random_free_udp_port](#randomfreeudpport) · [random_free_port](#randomfreeport) · [select_from_given_port](#selectfromgivenport) · [select_free_port](#selectfreeport)


## Documentation
### Port
`u16` type alias
```rust
pub type Port = u16;
```

### Selector
The `select_free_port` requires a structure passed in
```rust
pub struct Selector {
    // Check whether the port is available on TCP.
    // The default value is true.
    check_tcp: bool,
    // Check whether the port is available on UDP.
    // The default value is true.
    check_udp: bool,
    // Set the range of generated ports, default (0, 65525)
    port_range: (u16, u16),
    // Maximum number of random times. Default value: 100
    // If no available port number is found within the maximum random number of loops, None is returned
    max_random_times: u16,
}
```

### `is_free_tcp`
Check whether the port is not used on TCP
```rust
pub fn is_free_udp(port: Port) -> bool
```

### `is_free_udp`
Check whether the port is not used on UDP
```rust
pub fn is_free_udp(port: Port) -> bool
```

### `is_free`
Check whether the port is not used on TCP and UDP
```rust
pub fn is_free(port: Port) -> bool
```

### `random_free_tcp_port`
The system randomly assigns available TCP ports
```rust
pub fn random_free_tcp_port() -> Option<Port>
```

### `random_free_udp_port`
The system randomly assigns available UDP ports
```rust
pub fn random_free_udp_port() -> Option<Port>
```

### `random_free_port`
The system randomly assigns available TCP and UDP ports
```rust
pub fn random_free_port() -> Option<Port>
```

### `select_from_given_port`
Check from `given_port` and return the first available port

Return if `given_port` is available; Otherwise `given_port += given_port` until the port is available
```rust
pub fn select_from_given_port(given_port: Port) -> Option<Port>
```

### `select_free_port`
Gets a matching port based on the `Selector` parameter constraint
```rust
pub fn select_free_port(selector: Selector) -> Option<Port>
```

## Thanks
[portpicker-rs](https://github.com/Dentosal/portpicker-rs)

