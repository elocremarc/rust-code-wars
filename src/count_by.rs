// https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust

pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut my_vector: Vec<u32> = Vec::new();
    for count in 1..=n {
        my_vector.push(count * x);
    }
    my_vector
}
