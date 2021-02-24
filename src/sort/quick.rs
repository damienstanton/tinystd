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

//! [Quicksort][1]. Not bad.
//! [1]: https://en.wikipedia.org/wiki/Quicksort
use super::*;

pub struct Quick;

/// The well-known recursive sorting algorithm
pub fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
                return;
            }
        }
        _ => (),
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice must be non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            };
            right -= 1;
        }
    }

    // set the pivot to it's correct location
    let left = left + 1;
    slice.swap(0, left - 1);

    // recurse
    let (left, right) = slice.split_at_mut(left - 1);
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for Quick {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut items = vec![4, 2, 3, 1];
        Quick.sort(&mut items);
        assert_eq!(items, &[1, 2, 3, 4]);
    }
}
