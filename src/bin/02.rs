use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_lines = input.lines();
    let mut safe_lines: Vec<bool> = vec![];

    while let Some(line) = input_lines.next() {
        let safe = is_line_safe(line);
        safe_lines.push(safe);
    }

    // haha, &&bool...???? and also ofc im gonna check if true == true
    let num_of_safe_lines = safe_lines.iter().filter(|&&x| x == true).count() as u32;
    Some(num_of_safe_lines)
}

fn is_line_safe(line: &str) -> bool {
    let mut safe = true;
    let mut direction: Option<u32> = None;
    line.split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows::<(u32, u32)>()
        .for_each(|(i, i_plus_one)| {
            if !follows_rule_1(&mut safe, &mut direction, i, i_plus_one) {
                safe = false;
            }

            if !follows_rule_2(i, i_plus_one) {
                safe = false;
            }
        });

    safe
}

// RULE 1
// EITHER sequence[i+1] > sequence[i] for all i
// OR input[i+1] < sequence[i] for all i
fn follows_rule_1(safe: &mut bool, direction: &mut Option<u32>, i: u32, i_plus_one: u32) -> bool {
    if i_plus_one > i {
        match direction {
            Some(0) => *safe = false,
            _ => *direction = Some(1),
        }
    } else if i_plus_one < i {
        match direction {
            Some(1) => *safe = false,
            _ => *direction = Some(0),
        }
    } else {
        *safe = false; // i == i_plus_one violates RULE 1
    }
    *safe
}

// RULE 2
// ALSO 1 <= abs(input[i+1] - input[i]) <= 3 for all i?
fn follows_rule_2(i: u32, i_plus_one: u32) -> bool {
    let diff = u32::abs_diff(i, i_plus_one);
    (1..=3).contains(&diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_lines = input.lines();
    let mut safe_lines: Vec<bool> = vec![];

    while let Some(line) = input_lines.next() {
        let safe = is_line_safe_enough(line);
        safe_lines.push(safe);
    }

    let num_of_safe_lines = safe_lines.iter().filter(|&&x| x == true).count() as u32;
    Some(num_of_safe_lines)
}

fn is_line_safe_enough(line: &str) -> bool {
    //no need to check stuff if the list is safe already
    if is_line_safe(&line) {
        return true;
    }
    // line is now considered not safe according to RULE1 and RULE2
    let mut safe: bool = false;

    //NO LETS TEST RULE 3: for every i, is the line still not safe?
    let numbers: Vec<u32> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect();
    numbers.iter().enumerate().for_each(|(idx, i)| {
        let line_without_i = without_i(&line, idx);
        if is_line_safe(&line_without_i) {
            safe = true;
        }
    });
    safe
}

fn without_i(line: &str, _idx: usize) -> String {
    let line_without_i = line
        .split_whitespace()
        .enumerate()
        .filter_map(|(idx, val)| { // enumerations give you access to the index and the value for iteration!!
            if idx != _idx {
                Some(val)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join(" ");

    line_without_i
}


#[test]
fn test_without_i() {
    let line = "10 20 30 40 50";
    let result = without_i(line, 30);
    assert_eq!(result, "10 20 40 50");

    let line = "30";
    let result = without_i(line, 30);
    assert_eq!(result, "");

    let line = "";
    let result = without_i(line, 30);
    assert_eq!(result, "");
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
