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
use std::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

/// References can exist in one of three states:
/// - `Unshared` means the reference has not been given out at all
/// - `Exclusive` means exactly one owner has the reference
/// - `Shared(n)` means the reference has been given out to `n` borrowers
#[derive(Copy, Clone)]
pub enum RefState {
    Unshared,
    Exclusive,
    Shared(usize),
}

/// RefCell is a **non** thread-safe container for T
pub struct RefCell<T> {
    /// UnsafeCell is needed as it is the core interior mutability type
    value: UnsafeCell<T>,
    /// RefState is wrapped in a std::cell::Cell to allow mutation via &Self
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    /// wrap `T` in a reference cell
    pub fn from(value: T) -> Self {
        Self {
            value: UnsafeCell::from(value),
            state: Cell::from(RefState::Unshared),
        }
    }

    /// Return an immutable shared reference to `T`, IFF there exists no other
    /// _exclusive_ references to `T` already given out
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    /// Return a **mutable ** shared reference to `T`, IFF there exists no other
    /// _exclusive_ references to `T` already given out
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}
/// Ref is a custom wrapper that provides the correct `Drop` impl for T, as
/// opposed to simply returning `&T` or `&mut T`
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

/// Explain the drop semantics for `RefMut<T>`
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => unreachable!(),
            RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

/// Explain how to dereference a `T` from an `Ref<T>`
impl<T> Deref for Ref<'_, T> {
    type Target = T;

    /// This effectively is what makes `Ref` a smart pointer; the type is able
    /// to correcty `Drop` or hand out the shared reference to its interior
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

/// A `Mut`able `Ref`erence to `T`
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

/// Explain the drop semantics for `RefMut<T>`
impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => {}
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
        }
    }
}

/// Explain how to _immutably_ dereference a `T` from an `RefMut<T>`
impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    /// This effectively is what makes `Ref` a smart pointer; the type is able
    /// to correcty `Drop` or hand out the shared reference to its interior
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

/// Explain how to _mutably_ dereference a `T` from an `RefMut<T>`
impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}
