#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

mod segtree;
mod gcd_lcm;
mod template;
mod permutation;
mod crt;
mod binary_search;
mod modint;
mod dsu;
mod geo;

#[cfg(test)]
mod tests {
    use binary_search::BinarySearch;

    use super::*;

    #[test]
    fn permutation() {
        let mut p = permutation::Permutation::new((0..3).collect());
        assert_eq!(p.next(), Some(vec![0, 1, 2]));
        assert_eq!(p.next(), Some(vec![0, 2, 1]));
        assert_eq!(p.next(), Some(vec![1, 0, 2]));
        assert_eq!(p.next(), Some(vec![1, 2, 0]));
        assert_eq!(p.next(), Some(vec![2, 0, 1]));
        assert_eq!(p.next(), Some(vec![2, 1, 0]));
        assert_eq!(p.next(), None);
    }

    #[test]
    fn binary_search() {
        let v = vec![1, 2, 2, 3];
        assert_eq!(v.lower_bound(2), 1);
        assert_eq!(v.upper_bound(2), 3);
    }
}
