Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<h1 align="center">port-selector</h1>
<div align="center">
    
[![Build Status](https://img.shields.io/crates/v/port-selector)](https://crates.io/crates/port-selector)
![Crates Downloads](https://img.shields.io/crates/d/port-selector)
![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/port-selector-rs)
    
</div>
<div align="center">
    
[![Docs](https://img.shields.io/docsrs/port-selector)](https://docs.rs/port-selector/0.1.1/port_selector/)
[![GitHub Actions CI](https://img.shields.io/github/workflow/status/ZingerLittleBee/port-selector-rs/Test%20CI)](https://github.com/ZingerLittleBee/port-selector-rs/actions)
[![LICENSE](https://img.shields.io/crates/l/port-selector)](./LICENSE)
    
</div>

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

fn -> [is_free_tcp](#is_free_tcp) · [is_free_udp](#is_free_udp) · [is_free](#is_free) · [random_free_tcp_port](#random_free_tcp_port) · [random_free_udp_port](#random_free_udp_port) · [random_free_port](#random_free_port) · [select_from_given_port](#select_from_given_port) · [select_free_port](#select_free_port)


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
    pub check_tcp: bool,
    // 是否检查端口在 udp 上可用, 默认值 true
    pub check_udp: bool,
    // 设置生成的端口范围, 默认值 (0, 65525)
    pub port_range: (u16, u16),
    // 最大随机次数, 默认值 100
    // 如果在最大随机次数的循环之内都没有找到可用端口号, 则返回 None
    pub max_random_times: u16,
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
