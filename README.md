Language : 🇺🇸 English | [🇨🇳 简体中文](./README.zh-CN.md)

<h1 align="center">port-selector</h1>
<div align="center">

[![Build Status](https://img.shields.io/crates/v/port-selector)](https://crates.io/crates/port-selector)
![Crates Downloads](https://img.shields.io/crates/d/port-selector)
![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/port-selector-rs)

</div>
<div align="center">

[![Docs](https://img.shields.io/docsrs/port-selector)](https://docs.rs/port-selector/0.1.6/port_selector/)
[![GitHub Actions CI](https://img.shields.io/github/workflow/status/ZingerLittleBee/port-selector-rs/Test%20CI)](https://github.com/ZingerLittleBee/port-selector-rs/actions)
[![LICENSE](https://img.shields.io/crates/l/port-selector)](./LICENSE)

</div>

## Overview
port-selector is a rust library that provides functions to **port availability checking**, **port filtering based on conditions** and  **occupy specified ports**.

## Installation
1. Get the latest version -> https://crates.io/crates/port-selector

2. Add the dependent
```toml
[dependencies]
port-selector = "0.1.6"
```

3. Usage
```rust
use port_selector::{is_free, Port};
use port_selector::take_up::{random_take_up_port, take_up_port};

fn main() {
    // check
    let check_port: Port = 3000;
    println!("is_free({}) = {}", check_port, is_free(check_port));
    // take up
    let used_port = random_take_up_port();
    println!("used_port: {}", used_port);
    assert!(take_up_port(5000));
}
```

## Goods
type -> [Port](#port) · [Selector](#selector)

fn -> [is_free_tcp](#is_free_tcp) · [is_free_udp](#is_free_udp) · [is_free](#is_free) · [random_free_tcp_port](#random_free_tcp_port) · [random_free_udp_port](#random_free_udp_port) · [random_free_port](#random_free_port) · [select_from_given_port](#select_from_given_port) · [select_free_port](#select_free_port)

mod (`take_up`) -> [take_up_tcp_port](#take_up_tcp_port) · [take_up_udp_port](#take_up_udp_port) · [take_up_port](#take_up_port) · [random_take_up_tcp_port](#random_take_up_tcp_port) · [random_take_up_udp_port](#random_take_up_udp_port) · [random_take_up_port](#random_take_up_port)


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
    pub check_tcp: bool,
    // Check whether the port is available on UDP.
    // The default value is true.
    pub check_udp: bool,
    // Set the range of generated ports, default (0, 65525)
    pub port_range: (u16, u16),
    // Maximum number of random times. Default value: 100
    // If no available port number is found within the maximum random number of loops, None is returned
    pub max_random_times: u16,
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

Return if `given_port` is available; Otherwise `given_port += 1` until the port is available
```rust
pub fn select_from_given_port(given_port: Port) -> Option<Port>
```

### `select_free_port`
Gets a matching port based on the `Selector` parameter constraint
```rust
pub fn select_free_port(selector: Selector) -> Option<Port>
```

----
> The ports occupied by the `take_up` series of methods will be automatically released after the main function call ends.
### `take_up_tcp_port`
Occupy port on tcp
```rust
fn take_up_tcp_port(port: Port) -> bool
```

### `take_up_udp_port`
Occupy port on udp
```rust
fn take_up_udp_port(port: Port) -> bool
```

### `take_up_port`
Occupy port on tcp && udp
```rust
fn take_up_port(port: Port) -> bool
```

### `random_take_up_tcp_port`
Randomly occupied port on tcp by the system
```rust
fn random_take_up_tcp_port() -> Port
```

### `random_take_up_udp_port`
Randomly occupied port on udp by the system
```rust
fn random_take_up_udp_port() -> Port
```

### `random_take_up_port`
Randomly occupy tcp && udp ports by the system
```rust
fn random_take_up_port() -> Port
```

## Thanks
[portpicker-rs](https://github.com/Dentosal/portpicker-rs)