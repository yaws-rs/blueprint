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

[yaws book]: https://yaws-rs.github.io/book/
[yaoi]: https://github.com/yaws-rs/yaoi
[h11spec]: https://github.com/yaws-rs/h11spec
[tls]: https://github.com/yaws-rs/tls
[https]: https://github.com/yaws-rs/yaoi/blob/main/examples/blueprint-tls-http/src/main.rs