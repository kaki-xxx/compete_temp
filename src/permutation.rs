use cargo_snippet::snippet;

pub use permutation::Permutation;

#[snippet("_permutation", prefix = "use permutation::Permutation;")]
mod permutation {
    pub struct Permutation<T: Ord + Clone> {
        v: Vec<T>,
        ended: bool,
    }

    impl<T: Ord + Clone> Permutation<T> {
        pub fn new(v: Vec<T>) -> Permutation<T> {
            Permutation { v: v, ended: false }
        }
    }

    impl<T: Ord + Clone> Iterator for Permutation<T> {
        type Item = Vec<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.ended {
                return None;
            }
            let v = &mut self.v;
            let ret = v.clone();
            let mut k = v.len() - 1;
            while k > 0 && v[k - 1] > v[k] {
                k -= 1;
            };
            if k == 0 {
                v.reverse();
                self.ended = true;
            } else {
                k -= 1;
                let mut l = v.len() - 1;
                while l > k && v[k] > v[l] {
                    l -= 1;
                }
                v.swap(k, l);
                v[k + 1..].reverse();
            }
            Some(ret)
        }
    }
}
