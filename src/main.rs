use crate::in_place_algorithm::reverse_array;
mod in_place_algorithm;

fn main() {
    let mut data: Vec<Vec<u32>> = vec![vec![1,2,3], vec![4, 5, 6], vec![7, 8, 9]];
    dbg!(reverse_array(&mut data));
}
