extern crate core;

use regex::{Regex};

advent_of_code::solution!(1);

const VERTICAL_LIST_LEFT_SIDE: usize = 1; // why use usize, not i32??? what with the get function in the vaputers
const VERTICAL_LIST_RIGHT_SIDE: usize = 2;

pub fn part_one(input: &str) -> Option<u32> {
    //println!("input: {:?}", input);
    // let mut mut_input = input.to_string();
    let regex: Regex = Regex::new(r"(\d+)\s{3}(\d+)").unwrap();
    let mut list1: Vec<u32> = vec![]; //no difference with below...??
    let mut list2: Vec<u32> = Vec::new();

    let mut input_lines = input.lines(); // Create an iterator over lines

    while let Some(line) = input_lines.next() {
        if let Some(captures) = regex.captures(line) {
            if let Some(left_side_capture_group) = captures.get(VERTICAL_LIST_LEFT_SIDE) {
                list1.push(left_side_capture_group.as_str().parse::<u32>().ok()?);
            }
            if let Some(right_side_capture_group) = captures.get(VERTICAL_LIST_RIGHT_SIDE) {
                list2.push(right_side_capture_group.as_str().parse::<u32>().ok()?);
            }
        }
    }

    //init the slices that will be passed to the distance function
    let slice1: &[u32] = &list1;
    let slice2: &[u32] = &list2;

    Some(distance_between_lists(slice1, slice2))
}

fn distance_between_lists(list1: &[u32], list2: &[u32]) -> u32 {
    //println!("List 1: {:?}", list1);
    //println!("List 2: {:?}", list2);
    let mut mut_list1 = list1.to_vec();
    let mut mut_list2 = list2.to_vec();

    //find min_a in list1. find min_b in list2 subtract them
    //add that to the distance between list1 without min_a, and list2 without min_b
    //1 pass per list to get minimum
    if let (Some(&min_a), Some(&min_b)) = (mut_list1.iter().min(), mut_list2.iter().min()) {
        //second pass just to get the minimum numbers index lol
        let min_a_index = mut_list1.iter().position(|&x| x == min_a).unwrap();
        let min_b_index = mut_list2.iter().position(|&x| x == min_b).unwrap();
        // .position(|address_of_x|, predicate(x)) syntax I FINALLY GET IT!!!!

        // let min_index = list.iter()
        // .enumerate()
        // .min_by_key(|&(_, value)| value)
        // .map(|(index, _)| index);

        mut_list1.remove(min_a_index);
        mut_list2.remove(min_b_index);

        return u32::abs_diff(min_b, min_a) + distance_between_lists(&mut_list1, &mut_list2);
    }

    0
}


pub fn part_two(input: &str) -> Option<u32> {
    use std::collections::HashMap;

    let regex: Regex = Regex::new(r"(\d+)\s{3}(\d+)").unwrap();
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    let mut input_lines = input.lines();

    while let Some(line) = input_lines.next() {
        if let Some(captures) = regex.captures(line) {
            if let Some(left_side_capture_group) = captures.get(VERTICAL_LIST_LEFT_SIDE) {
                list1.push(left_side_capture_group.as_str().parse::<u32>().ok()?);
            }
            if let Some(right_side_capture_group) = captures.get(VERTICAL_LIST_RIGHT_SIDE) {
                list2.push(right_side_capture_group.as_str().parse::<u32>().ok()?);
            }
        }
    }

    // Count occurrences of each number in list2
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    for &num in &list2 {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score = list1
        .iter()
        .map(|&num| num * count_map.get(&num).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
