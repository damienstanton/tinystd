# 🤏 tinystd

[![CI][1]][CI]
![Rust toolchain support][2]
[![Documentation on docs.rs][3]][Docs]

This crate is a place where I take notes when learning about a particular data structure or algorithm in Rust, usually
by trying to implement a nominal version of the thing myself.

Types implemented, in no particular order:

- [x] `HashMap<K, V>`
- [x] `RefCell<T>`
- [x] `Rc<T>`

TODO:

- [ ] Unit tests for `RefCell`
- [ ] Unit tests for `Rc`
- Non-threadsafe things? E.g. `Arc` or `RwLock`?

© 2020 Damien Stanton

See LICENSE for details.

[1]: https://github.com/damienstanton/tinystd/workflows/CI/badge.svg
[2]: https://img.shields.io/badge/Rust%20toolchain-stable-%23DEA484?style=plastic&logo=rust
[3]: https://docs.rs/tinystd/badge.svg?version=0.1.0
[CI]: https://github.com/damienstanton/tinystd/actions
[Docs]: https://docs.rs/tinystd/0.1.0/tinystd/
