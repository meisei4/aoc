use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
   capture_multiply_add(input)
}

const MUL_OP: usize = 0; //mul(
const OPERAND_1: usize = 1;
const OPERAND_2: usize = 2;
// ) right brace = 3

fn capture_multiply_add(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").ok()?;
    let mut sum: u32 = 0;

    for capture_group in regex.captures_iter(input) {
        let operand_1 = capture_group.get(OPERAND_1)?.as_str();
        let operand_2 = capture_group.get(OPERAND_2)?.as_str();

        let num_1: u32 = operand_1.parse::<u32>().ok()?;
        let num_2: u32 = operand_2.parse::<u32>().ok()?;
        sum += num_1 * num_2;
    }
    Some(sum)
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
