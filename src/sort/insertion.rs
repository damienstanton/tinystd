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
//! [Insertion sort][1]. Not very good.
//! [1]: https://en.wikipedia.org/wiki/Insertion_sort
use super::*;

pub struct Insertion {
    /// "Smart" in the sense that, when true, `Insertion` will use binary search
    /// instead of a naive traversal to determine the current swap index `i`
    pub smart: bool,
}
impl Sorter for Insertion {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i, i - 1);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,  // match found, returning found index
                    Err(i) => i, // no match, returning appropriate index
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_works() {
        let mut items = vec![4, 2, 3, 1];
        Insertion { smart: true }.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
    #[test]
    fn not_smart_works() {
        let mut items = vec![4, 2, 3, 1];
        Insertion { smart: false }.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
}
