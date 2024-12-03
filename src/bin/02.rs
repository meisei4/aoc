use itertools::{Itertools};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_lines = input.lines(); // Create an iterator over lines
    let mut safe_lines: Vec<bool> = vec![];

    while let Some(line) = input_lines.next() {
        let mut safe: bool = true;
        let mut direction: Option<u32> = None; // None = undecided, 0 = decreasing, 1 = increasing
        line.split_whitespace()// split by whitespace
            .filter_map(|num| num.parse::<u32>().ok()) //parse into u32's
            .tuple_windows::<(u32, u32)>() // itertools trick to iterate through adjacent elements
            .for_each(|(i, i_plus_one)| { // for each adjacent element... each tuple_window :^)
                // RULE 1
                // EITHER sequence[i+1] > sequence[i] for all i
                // OR input[i+1] < sequence[i] for all i
                if i_plus_one > i {
                    match direction {
                        Some(0) => safe = false, // cant switch direction (would call "break;" here but cant in closure
                        _ => direction = Some(1), // in the case of the first time assigning direction
                    }
                } else if i_plus_one < i {
                    match direction {
                        Some(1) => safe = false, // cant switch direction
                        _ => direction = Some(0), // in the case of the first time assigning direction
                    }
                } else {
                    // i == i_plus_one violates RULE 1
                    safe = false;
                }

                // RULE 2
                // ALSO 1 <= abs(input[i+1] - input[i]) <= 3 for all i?
                let diff = u32::abs_diff(i_plus_one, i);
                if diff < 1 || diff > 3 {
                    safe = false; // Difference not in [1, 3]
                }
            });
        safe_lines.push(safe);
    }

    // haha, &&bool...???? and also ofc im gonna check if true == true
    let num_of_safe_lines: u32 = safe_lines.iter().filter(|&&x| x == true).count() as u32;
    Some(num_of_safe_lines)
}


pub fn part_two(input: &str) -> Option<u32> {
    None
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
