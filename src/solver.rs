#[macro_export]
macro_rules! solution {
    ($expression:expr) => {
        pub fn solve(path: PathBuf) {
            ($expression)(path)
        }
    };
}
