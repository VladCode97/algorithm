use crate::in_place_algorithm::reverse_array;
use crate::group_by::group_by_name_course;

mod in_place_algorithm;
mod group_by;

fn main() {
    let mut data: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    dbg!(reverse_array(&mut data));
    dbg!(group_by_name_course());
}
