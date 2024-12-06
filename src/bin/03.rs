use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    capture_multiply_add(input)
}

// const MUL_OP: usize = 0; // mul(
const OPERAND_1: usize = 1;
const OPERAND_2: usize = 2;
// ) right brace = 3

macro_rules! op1 {
    () => { r"\d{1,3}" };
}
macro_rules! op2 {
    () => { r"\d{1,3}" };
}
macro_rules! simple_mul_pattern {
    () => {
        concat!("mul\\(", op1!(), ",", op2!(), "\\)")
    };
}

fn capture_multiply_add(input: &str) -> Option<u32> {
    let regex = Regex::new(simple_mul_pattern!()).ok()?;
    let mut sum: u32 = 0;

    for capture_group in regex.captures_iter(input) {
        let operand_1 = capture_group.get(OPERAND_1)?.as_str();
        let operand_2 = capture_group.get(OPERAND_2)?.as_str();

        let num_1: u32 = operand_1.parse().ok()?;
        let num_2: u32 = operand_2.parse().ok()?;
        sum += num_1 * num_2;
    }
    Some(sum)
}




pub fn part_two(input: &str) -> Option<u32> {
    capture_multiply_add_with_control(input)
}

macro_rules! op1_name {
    () => { "op1" };
}
macro_rules! op2_name {
    () => { "op2" };
}
macro_rules! do_name {
    () => { "do" };
}
macro_rules! dont_name {
    () => { "dont" };
}
macro_rules! mul_name {
    () => { "mul" };
}
macro_rules! mul_pattern {
    () => {
        concat!("mul\\((?P<", op1_name!(), ">", op1!(), "),(?P<", op2_name!(), ">", op2!(), ")\\)")
    };
}
macro_rules! do_pattern {
    () => {
        concat!("(?P<", do_name!(), ">do\\(\\))")
    };
}
macro_rules! dont_pattern {
    () => {
        concat!("(?P<", dont_name!(), ">don't\\(\\))")
    };
}
macro_rules! mul_with_control {
    () => {
        concat!(
            "(?P<", mul_name!(), ">", mul_pattern!(), ")|",
            do_pattern!(), "|",
            dont_pattern!()
        )
    };
}
fn capture_multiply_add_with_control(input: &str) -> Option<u32> {
    let regex = Regex::new(mul_with_control!()).ok()?;
    let mut sum = 0;
    let mut mul_enabled = true;

    for captures in regex.captures_iter(input) {
        match (captures.name(mul_name!()), captures.name(do_name!()), captures.name(dont_name!())) {
            (Some(_), None, None) => {
                let num_1: u32 = captures.name(op1_name!())?.as_str().parse().ok()?;
                let num_2: u32 = captures.name(op2_name!())?.as_str().parse().ok()?;
                if mul_enabled {
                    sum += num_1 * num_2;
                }
            }
            (None, Some(_), None) => {
                mul_enabled = true;
            }
            (None, None, Some(_)) => {
                mul_enabled = false;
            }
            _ => {}
        }
    }
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
