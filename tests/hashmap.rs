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
use tinystd::collections::HashMap;

#[test]
fn basic() {
    let mut m = HashMap::<i32, i32>::new();
    eprintln!("Checking insertions...");
    m.insert(1, 1);
    m.insert(2, 2);
    assert_eq!(m.get(1), Some(&1));
    assert_eq!(m.get(3), None);

    eprintln!("Checking updates...");
    m.insert(2, 1);
    assert_eq!(m.get(2), Some(&1));

    eprintln!("Checking deletion...");
    m.remove(2);
    assert_eq!(m.get(2), None);
}
