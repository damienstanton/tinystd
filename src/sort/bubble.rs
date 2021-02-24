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
//! [Bubblesort][1]. Not good.
//! [1]: https://en.wikipedia.org/wiki/Bubble_sort
use super::*;

pub struct Bubble;
impl Sorter for Bubble {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i, i - 1);
                    swapped = true;
                }
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
        Bubble.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
}
