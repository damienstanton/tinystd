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

//! [Selection sort][1]. Not very good, worse than [`crate::sort::insertion`] but at least has `O(1)` space complexity.
//! [1]: https://en.wikipedia.org/wiki/Selection_sort
use super::*;

pub struct Selection;
impl Sorter for Selection {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let min_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice must be non-empty");
            if unsorted != min_rest {
                slice.swap(unsorted, min_rest);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut items = vec![4, 2, 3, 1];
        Selection.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
}
