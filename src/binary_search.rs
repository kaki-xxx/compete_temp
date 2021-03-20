use cargo_snippet::snippet;

pub use binary_search::BinarySearch;

#[snippet("_binary_search", prefix = "use binary_search::BinarySearch;")]
mod binary_search {
    pub trait BinarySearch<T: Ord> {
        fn lower_bound(&self, x: T) -> usize;
        fn upper_bound(&self, x: T) -> usize;
    }

    impl<T: Ord> BinarySearch<T> for Vec<T> {
        fn lower_bound(&self, x: T) -> usize {
            let mut ok = self.len();
            let mut ng = 1usize.wrapping_neg();
            while ok.wrapping_sub(ng) > 1 {
                let mid = ok.wrapping_add(ng) / 2;
                if x <= self[mid as usize] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }

        fn upper_bound(&self, x: T) -> usize {
            let mut ok = self.len();
            let mut ng = 1usize.wrapping_neg();
            while ok.wrapping_sub(ng) > 1 {
                let mid = ok.wrapping_add(ng) / 2;
                if x < self[mid as usize] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}
