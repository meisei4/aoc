use std::collections::{HashSet};
use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
        let rules: HashSet<String> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect();

    let comparator = |a: &String, b: &String| -> Ordering {
        let ab = format!("{}|{}", a, b);
        let ba = format!("{}|{}", b, a);
        if rules.contains(&ab) {
            Ordering::Less
        } else if rules.contains(&ba) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let page_lists: Vec<Vec<String>> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();

    let sum = page_lists
        .iter()
        .map(|list| format_correctly(list, &comparator))
        .filter(|(original, sorted)| original == sorted)
        .map(|(_, sorted)| midpoint(&sorted))
        .sum();

    Some(sum)
}

fn format_correctly<F>(list: &Vec<String>, comparator: F) -> (Vec<String>, Vec<String>)
where
    F: Fn(&String, &String) -> Ordering,
{
    let mut sorted = list.clone();
    sorted.sort_by(comparator);
    (list.clone(), sorted)
}

fn midpoint(list: &Vec<String>) -> u32 {
    if list.is_empty() {
        0
    } else {
        list[list.len() / 2].parse::<u32>().unwrap_or(0)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules: HashSet<String> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect();

    let comparator = |a: &String, b: &String| -> Ordering {
        let ab = format!("{}|{}", a, b);
        let ba = format!("{}|{}", b, a);
        if rules.contains(&ab) {
            Ordering::Less
        } else if rules.contains(&ba) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let page_lists: Vec<Vec<String>> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();

    let sum = page_lists
        .iter()
        .map(|list| format_correctly(list, &comparator))
        .filter(|(original, sorted)| original != sorted)
        .map(|(_, sorted)| midpoint(&sorted))
        .sum();

    Some(sum)
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
