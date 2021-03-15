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
//! Implementations of a few sorting algorithms, based on a brilliant
//! [Crust of Rust][1] stream by Jon Gjengset.
//!
//! [1]: https://youtu.be/h4RkCyJyXmM
mod bubble;
mod insertion;
mod quick;
mod selection;

pub use bubble::Bubble;
pub use insertion::Insertion;
pub use quick::Quick;
pub use selection::Selection;

/// A shared sorting trait
pub trait Sorter {
    /// Any mutable `ref<slice<T>>` that is `Ord` is sortable.
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A simple smoke check for the Sorter trait using [`slice::sort`]
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std() {
        let mut items = vec![4, 2, 3, 1];
        StdSorter.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
}
