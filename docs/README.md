# 🤏 tinystd

[![Documentation on docs.rs][3]][Docs]

This crate is a place where I take notes when learning about a particular data structure or algorithm in Rust, usually
by trying to implement a nominal version of the thing myself. 

Note that even though the crate is called `tinystd`, this
does _not_ mean that the crate will not arbitrarily have third-party dependencies in some cases.

Data structures implemented

- [x] `HashMap<K, V>`
- [x] `RefCell<T>`
- [x] `Rc<T>`
- [x] `channel::<T>`

Algorithms implemented:

- [x] `Bubblesort`
- [x] `Insertion sort (naive)`
- [x] `Insertion sort (using binary search)`
- [x] `Quicksort`

TODO:

- [ ] Unit tests for `RefCell`
- [ ] Unit tests for `Rc`
- [ ] Convert unit tests for sorting into better documentation (more literate style)
- Non-threadsafe things? E.g. `Arc` or `RwLock`?

© 2020-2021 Damien Stanton

See LICENSE for details.

[1]: https://github.com/damienstanton/tinystd/workflows/CI/badge.svg
[3]: https://docs.rs/tinystd/badge.svg?version=0.1.0
[CI]: https://github.com/damienstanton/tinystd/actions
[Docs]: https://docs.rs/tinystd
