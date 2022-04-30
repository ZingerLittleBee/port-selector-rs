Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

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

port-selector 是一个 rust 的库, 主要提供端口可用性检查和根据条件筛选端口的功能.

## Install
1. 获取最新版本 -> https://crates.io/crates/port-selector

2. 添加依赖
```toml
[dependencies]
port-selector = "x.x.x"
```

3. 使用
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
`u16` 类型别名
```rust
pub type Port = u16;
```

### Selector
`select_free_port` 函数需要传入的结构体
```rust
pub struct Selector {
    // 是否检查端口在 tcp 上可用, 默认值 true
    check_tcp: bool,
    // 是否检查端口在 udp 上可用, 默认值 true
    check_udp: bool,
    // 设置生成的端口范围, 默认值 (0, 65525)
    port_range: (u16, u16),
    // 最大随机次数, 默认值 100
    // 如果在最大随机次数的循环之内都没有找到可用端口号, 则返回 None
    max_random_times: u16,
}
```

### `is_free_tcp`
检查端口在 tcp 上是否未使用
```rust
pub fn is_free_udp(port: Port) -> bool
```

### `is_free_udp`
检查端口在 udp 上是否未使用
```rust
pub fn is_free_udp(port: Port) -> bool
```

### `is_free`
检查端口在 tcp && udp 上是否未使用
```rust
pub fn is_free(port: Port) -> bool
```

### `random_free_tcp_port`
由系统随机分配可用 tcp 端口
```rust
pub fn random_free_tcp_port() -> Option<Port>
```

### `random_free_udp_port`
由系统随机分配可用 udp 端口
```rust
pub fn random_free_udp_port() -> Option<Port>
```

### `random_free_port`
由系统随机分配可用 tcp && udp 端口
```rust
pub fn random_free_port() -> Option<Port>
```

### `select_from_given_port`
从 `given_port` 开始检查, 返回第一个可用端口

如果 `given_port` 可用, 则返回; 否则 `given_port += given_port`, 直到端口可用
```rust
pub fn select_from_given_port(given_port: Port) -> Option<Port>
```

### `select_free_port`
根据 `Selector` 参数约束获取一个满足条件的端口
```rust
pub fn select_free_port(selector: Selector) -> Option<Port>
```

## Thanks
[portpicker-rs](https://github.com/Dentosal/portpicker-rs)