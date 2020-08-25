// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Special note on the phantom/marker data:
//!
//! Without the `PhantomData` type, the compiler cannot know that the `Rc`
//! owns `T`. This matters when lifetimes are considered. Imagining:
//!
//! ```text
//! struct Foo<'a, T> {v: &mut T}
//! impl<T> Drop for Foo<T> {
//!     fn drop(&mut self) {
//!         self.v.do_some_stuff();
//!     }
//! }
//!
//! fn main() {
//!     let (foo, t);
//!     // remember, the compiler drops in reverse order of declaration
//!     let t: String::from("ok");
//!     let foo = Rc::from(Foo { v: &mut t });
//! }
//! ```
//! Without the marker in `Rc`, the compiler _cannot know_ that it still
//! holds the inner `T`. In other words, the marker tells the compiler to
//! treat the `Rc` as though it owns a `T`; Dropping `Rc` must be handled
//! (by the compiler) as though it may be dropping `T`. This is called the
//! "drop check". See [the nomicon][1] for details.
//!
//! [1]: https://doc.rust-lang.org/nomicon/dropck.html
use std::{cell::Cell, marker::PhantomData, ops::Deref, ptr::NonNull};
/// `Shared` is an inner container which holds the reference count
struct Shared<T> {
    value: T,
    refcount: Cell<usize>,
}

/// A `R`eference `c`ounted container for `T`
pub struct Rc<T> {
    inner: NonNull<Shared<T>>,
    _marker: PhantomData<Shared<T>>,
}

impl<T> Rc<T> {
    pub fn from(value: T) -> Self {
        let inner = Box::new(Shared {
            value,
            refcount: Cell::from(1),
        });
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

/// Note that we do not need `T` to be, itself, `Copy`
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let current = inner.refcount.get();
        inner.refcount.set(current + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

/// Explain how to dereference a `T` from an `Rc<T>`
impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

/// Explain the drop semantics for `Rc<T>`
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let current = inner.refcount.get();
        if current == 1 {
            // this is the last ref
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other shared refs already given out
            inner.refcount.set(current - 1);
        }
    }
}
