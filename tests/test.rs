#![feature(globs)]

extern crate IteratorExtras;
use IteratorExtras::*;

#[test]
fn test_stride() {

    let xs = vec![0u, 1, 2, 3, 4, 5];
    let mut stride = xs.into_iter().stride(3);
    assert_eq!(stride.size_hint(), (2, Some(2)));
    let strided: Vec<uint> = stride.collect();
    assert_eq!(strided, vec![0u, 3]);

    let xs = vec![0u, 1, 2, 3, 4, 5];
    let mut stride = xs.into_iter().stride(2);
    assert_eq!(stride.size_hint(), (3, Some(3)));
    let strided: Vec<uint> = stride.collect();
    assert_eq!(strided, vec![0u, 2, 4]);

    let xs = vec![0u, 1, 2, 3, 4, 5];
    let mut stride = xs.into_iter().stride(1);
    assert_eq!(stride.size_hint(), (6, Some(6)));
    let strided: Vec<uint> = stride.collect();
    assert_eq!(strided, vec![0u, 1, 2, 3, 4, 5]);

    let xs = vec![0u, 1, 2, 3, 4, 5];
    let mut stride = xs.into_iter().stride(0);
    assert_eq!(stride.size_hint(), (6, Some(6)));
    let strided: Vec<uint> = stride.collect();
    assert_eq!(strided, vec![0u, 1, 2, 3, 4, 5]);

}

#[test]
fn test_map_pairs() {

    let xs = vec![0i, 1, 5, 8, 10];
    let pairwise_diffs: Vec<int> = xs.into_iter().map_pairs(|[l,r]| r - l).collect();
    assert_eq!(pairwise_diffs, vec![1i, 3]);

}

#[test]
fn test_scan1() {
    let xs = vec![0i, 1, 3, 6, 10];
    let diffs: Vec<int> = xs.into_iter().scan1(|st, x| {
        let diff = x - *st;
        *st = x;
        Some(diff)
        }).collect();
    assert_eq!(diffs, vec![1i, 2, 3, 4]);
}
