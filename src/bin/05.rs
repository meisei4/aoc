use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let ordering_rules = generate_rules(input)?;
    let mut sum_of_middle_pages = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        //SILLY CHECK THAT I COULD PROBABLY AVID WITH A REGEX, but TOO LAZY
        if line.contains('|') {
            continue; //skip all the rules haha
        }
        let list_of_page_numbers: Vec<u32> = line
            .split(',')
            .filter_map(|page_number| page_number.trim().parse::<u32>().ok())
            .collect();
        println!("List of page numbers: {:?}", list_of_page_numbers);
        sum_of_middle_pages += get_middle_page(&list_of_page_numbers, &ordering_rules);
    }
    Some(sum_of_middle_pages)
}


fn generate_rules(input: &str) -> Option<HashMap<u32, HashSet<u32>>> {
    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut rules = input.lines();
    while let Some(rule) = rules.next() {
        //SILLY CHECK THAT I COULD PROBABLY AVID WITH A REGEX, but TOO LAZY
        if !rule.contains('|') {
            break;
        }
        let mut split = rule.trim().split('|');
        let num1_str = split.next()?;
        let num2_str = split.next()?;

        let num1: u32 = num1_str.parse::<u32>().ok()?;
        let num2: u32 = num2_str.parse::<u32>().ok()?;
        // add num1 as a page number that can only appear BEFORE num2
        rules_map
            .entry(num2)
            .or_insert_with(HashSet::new)
            .insert(num1);
    }
    println!("Rules: {:?}", rules_map);
    Some(rules_map)
}

fn get_middle_page(list_of_page_numbers: &Vec<u32>, ordering_rules: &HashMap<u32, HashSet<u32>>, ) -> u32 {
    if list_of_page_numbers.iter().enumerate().all(|(idx, &current_page)| {
        if let Some(order_rules) = ordering_rules.get(&current_page) {
            order_rules.iter().all(|&page_that_can_only_occur_before| {
                list_of_page_numbers.iter().position(|&page| page == page_that_can_only_occur_before)
                    .map(|found_page_index| {
                        // when encountering a page in the ruleset for the current page:
                        // return whether it follows the rule of appearing BEFORE the current pages idx or not
                        found_page_index < idx
                    }).unwrap_or(true)
            })
        } else {
            true
        }
    }) {
        *list_of_page_numbers.get(list_of_page_numbers.len() / 2).unwrap_or(&0)
    } else {
        0
    }
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
