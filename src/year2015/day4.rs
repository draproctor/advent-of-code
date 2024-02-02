use std::fs::read_to_string;
use std::path::PathBuf;

use crate::solution;

solution!(|path| {
    let key_prefix = read_to_string(path).unwrap();
    let result = lowest_md5_prefix(&key_prefix, 5);
    println!("The lowest number is {result} for 5 zeroes");
    let result = lowest_md5_prefix(&key_prefix, 6);
    println!("The lowest number is {result} for 6 zeroes");
});

fn lowest_md5_prefix(prefix: &str, zero_count: usize) -> u32 {
    let mut suffix = 0;
    loop {
        let input = format!("{prefix}{suffix}");
        let hash = format!("{:x}", md5::compute(input));
        let num_leading_zeroes = hash.bytes().into_iter().take_while(|b| b == &b'0').count();
        if num_leading_zeroes == zero_count {
            break;
        }
        suffix += 1;
    }

    suffix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(lowest_md5_prefix("abcdef", 5), 609043);
    }
}
