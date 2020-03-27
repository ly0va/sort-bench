use std::env;
use std::time::{SystemTime, Duration};
use rand::{Rng, StdRng, SeedableRng};
use rayon::prelude::*;

const LENGTH: usize = 100_000_000;

fn bench<F>(size: usize, sorter: F) -> Duration 
    where F: Fn(&mut [i32]) {
    let seed = [18; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut unsorted = Vec::new();
    unsorted.resize_with(size, || rng.gen::<i32>());
    let now = SystemTime::now();
    sorter(&mut unsorted);
    now.elapsed().unwrap()
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let len = argv.get(1).unwrap_or(&LENGTH.to_string())
                  .parse().unwrap_or(LENGTH);
    println!("Number of CPUs:  {}", num_cpus::get());
    println!("Single-threaded: {:?}", bench(len, |a| { merge_sort(a); }));
    println!("Rayon's join:    {:?}", bench(len, |a| { join_merge_sort(a); }));
    println!("Builtin stable:  {:?}", bench(len, |a| { a.sort(); }));
    println!("Rayon stable:    {:?}", bench(len, |a| { a.par_sort(); }));
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        arr.to_vec()
    } else {
        let mid = arr.len() / 2;
        let left_half = merge_sort(&arr[..mid]);
        let right_half = merge_sort(&arr[mid..]);
        merge(&left_half, &right_half)
    }
}

fn join_merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= (1 << 13) {  // sequential fallback
        merge_sort(arr)
    } else {
        let mid = arr.len() / 2;
        let (left_half, right_half) = rayon::join(
            || join_merge_sort(&arr[mid..]), 
            || join_merge_sort(&arr[..mid])
        );
        merge(&left_half, &right_half)
    }
}

// alternative version of merge, join_merge_sort runs faster with it
fn _merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend(&arr1[i..]);
    result.extend(&arr2[j..]);
    result
}

fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = vec![0; arr1.len() + arr2.len()];
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result[i+j] = arr1[i];
            i += 1;
        } else {
            result[i+j] = arr2[j];
            j += 1;
        }
    }
    if i < arr1.len() {
        result[i+j..].copy_from_slice(&arr1[i..]);
    } else if j < arr2.len() {
        result[i+j..].copy_from_slice(&arr2[j..]);
    }
    result
}
