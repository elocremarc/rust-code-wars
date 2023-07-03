// https://www.codewars.com/kata/515e271a311df0350d00000f/rust

pub fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for v in &vec {
        sum += v * v;
    }
    sum
}
