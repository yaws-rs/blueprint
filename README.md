# blueprint (trait)

Sans-io Static no_std no-allocation protocol / state machine blueprints.

See [yaws book] for more.

# Runtimes

- [yaoi] batch-first io_uring incremental hugetable static sans-I/O no-allocations

# Layers / Apps

- [h11spec] no_std, no-alloc, HTTP/1.1 Spec
- [tls] rustls layer

# Examples

- [https] https pipeline using h11spec and tls

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[yaws book]: https://yaws-rs.github.io/book/
[yaoi]: https://github.com/yaws-rs/yaoi
[h11spec]: https://github.com/yaws-rs/h11spec
[tls]: https://github.com/yaws-rs/tls
[https]: https://github.com/yaws-rs/yaoi/blob/main/examples/blueprint-tls-http/src/main.rs
