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
//! Implementations of [`std::sync::mpsc`]-like channels, based on a
//! [Crust of Rust][1] stream by Jon Gjengset.
//!
//! [1]: https://youtu.be/b4mS5UPHh20
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

/// Tx half of the channel
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }

    pub fn send_many(&mut self, ts: Vec<T>) {
        let mut tds = VecDeque::new();
        for t in ts {
            tds.push_back(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.append(&mut tds);
        drop(inner);
        self.shared.available.notify_one();
    }
}

// NOTE: manual impl of Clone required to avoid ambiguity in whether the clone
// method clones the `Arc<T>` (which we want) or the `T` (which would require
// that `T: Clone`, and this is the assumption that the `#[derive(Clone)] macro
// itself makes.
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let solo = inner.senders == 0;
        drop(inner);
        if solo {
            self.shared.available.notify_one();
        }
    }
}

/// Rx half of the channel
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

/// The channel data container
struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}
/// A container used for signaling
struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

/// The primary constructor for a [tinystd::channel].
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn send_recv() {
        let (mut tx, mut rx) = channel();
        let x = 10;
        tx.send(x);
        assert_eq!(rx.recv(), Some(x));
    }
    #[test]
    fn send_on_close() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
    #[test]
    fn recv_on_close() {
        // TODO: just a copy of send for now
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
}
