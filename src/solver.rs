use std::path::PathBuf;

pub trait Solver {
    fn solve(&self, path: PathBuf);
}

#[macro_export]
macro_rules! solution {
    ($expression:expr) => {
        use crate::solver::Solver;

        pub struct Solution;

        impl Solver for Solution {
            fn solve(&self, path: PathBuf) {
                $expression(path)
            }
        }
    };
}
