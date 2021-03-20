use cargo_snippet::snippet;

pub use dsu::DSU;

#[snippet("_dsu", prefix = "use dsu::DSU;")]
mod dsu {
    use std::mem::swap;

    pub struct DSU {
        parents: Vec<i64>,
    }

    impl DSU {
        pub fn new(n: usize) -> DSU {
            DSU { parents: vec![-1; n] }
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let mut px = self.root(x);
            let mut py = self.root(y);
            if px == py {
                return;
            }
            if -self.parents[x] < -self.parents[y] {
                swap(&mut px, &mut py);
            }
            self.parents[px] += self.parents[py];
            self.parents[py] = px as i64;
        }

        pub fn root(&mut self, x: usize) -> usize {
            let px = self.parents[x];
            if px < 0 {
                return x;
            }
            self.parents[x] = self.root(px as usize) as i64;
            self.parents[x] as usize
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        
        pub fn size(&mut self, x: usize) -> usize {
            let px = self.root(x);
            -self.parents[px as usize] as usize
        }
    }
}
