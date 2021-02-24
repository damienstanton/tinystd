use rand::prelude::*;
use std::{cell::Cell, rc::Rc};
use tinystd::sort::*;

/// A basic structure used to evaluate different sorting algorithms based on
/// the number of comparisons made. This does _not_ match up 1:1 with `O()`
/// complexity analysis, but it is nonetheless a good performance heuristic.
#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    comps: Rc<Cell<usize>>,
}

// The long set of boilerplate that follow implement the required comparison
// trait methods for our `SortEvaluator`
impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.comps.set(self.comps.get() + 1);
        self.t == other.t
    }
}

impl<T> PartialOrd for SortEvaluator<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.comps.set(self.comps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T> Ord for SortEvaluator<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.comps.set(self.comps.get() + 1);
        self.t.cmp(&other.t)
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

// The benchmarking begins below.
fn run_bench<T, S>(sorter: S, values: &[SortEvaluator<T>], counter: &Cell<usize>) -> (usize, f64)
where
    T: Ord + Clone,
    S: Sorter,
{
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm,n,comparisons,time"); // header
    for &n in &[0, 1, 10, 100, 1000, 10_000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                comps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            // data for each row
            let took = run_bench(Bubble, &values, &counter);
            println!("{},{},{},{}", "bubble", n, took.0, took.1);
            let took = run_bench(Insertion { smart: true }, &values, &counter);
            println!("{},{},{},{}", "insertion-smart", n, took.0, took.1);
            let took = run_bench(Insertion { smart: false }, &values, &counter);
            println!("{},{},{},{}", "insertion-not-smart", n, took.0, took.1);
            let took = run_bench(Selection, &values, &counter);
            println!("{},{},{},{}", "selection", n, took.0, took.1);
            let took = run_bench(Quick, &values, &counter);
            println!("{},{},{},{}", "quick", n, took.0, took.1);
        }
    }

    Ok(())
}
