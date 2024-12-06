use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    identify_and_count_xmas_occurrences(input)
}

const TARGET_WORD: &str = "XMAS";
// THIS REQUIRES isize because negative directions
const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn identify_and_count_xmas_occurrences(input: &str) -> Option<u32> {
    let grid: Vec<&str> = input.lines().collect();
    let mut visited = HashMap::new();
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited.contains_key(&(x,y)) { continue; }
            if &grid[y][x..x+1] == "X" {
                total += flood_fill_search(&grid, x, y, &mut visited);
            }
        }
    }
    Some(total)
}

fn flood_fill_search(grid: &[&str], start_x: usize, start_y: usize, visited: &mut HashMap<(usize,usize),bool>) -> u32 {
    let mut stack = vec![(start_x,start_y)];
    let mut found = 0;

    while let Some((x,y)) = stack.pop() {
        if visited.contains_key(&(x,y)) { continue; }
        visited.insert((x,y),true);
        if &grid[y][x..x+1] == "X" {
            found += check_all_dirs(grid, x, y);
        }
        for (nx,ny) in get_surrounding_cells(grid, x, y) {
            if !visited.contains_key(&(nx,ny)) {
                stack.push((nx,ny));
            }
        }
    }
    found
}

fn check_all_dirs(grid: &[&str], x: usize, y: usize) -> u32 {
    let mut found_word_count = 0;
    for (dx,dy) in DIRECTIONS {
        if find_word(grid, x, y, dx, dy) {
            found_word_count += 1;
        }
    }
    found_word_count
}

fn find_word(grid: &[&str], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    for i in 0..TARGET_WORD.len() {
        let xx = x as isize + dx*(i as isize);
        let yy = y as isize + dy*(i as isize);
        if xx < 0 || yy < 0 { return false; }
        let xx = xx as usize;
        let yy = yy as usize;
        if yy >= grid.len() || xx >= grid[yy].len() { return false; }
        if &grid[yy][xx..xx+1] != &TARGET_WORD[i..i+1] {
            return false;
        }
    }
    true
}

fn get_surrounding_cells(grid: &[&str], x: usize, y: usize) -> Vec<(usize,usize)> {
    let mut cells = Vec::new();
    for (dx, dy) in DIRECTIONS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 { continue; }
        let nx = nx as usize;
        let ny = ny as usize;
        if ny < grid.len() && nx < grid[ny].len() {
            cells.push((nx,ny));
        }
    }
    cells
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<&str> = input.lines().collect();
    Some(count_x_mas_patterns(&grid))
}

fn count_x_mas_patterns(grid: &[&str]) -> u32 {
    let mut count = 0;

    for y in 1..grid.len().saturating_sub(1) {
        for x in 1..grid[y].len().saturating_sub(1) {
            if is_x_mas(&grid, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn is_mas_or_sam(triple: &[char]) -> bool {
    (triple == &['M', 'A', 'S']) || (triple == &['S', 'A', 'M'])
}

fn is_x_mas(grid: &[&str], cx: usize, cy: usize) -> bool {
    let chars = |x: usize, y: usize| grid[y][x..x+1].chars().next().unwrap();

    let top_left = chars(cx-1, cy-1);
    let top_right = chars(cx+1, cy-1);
    let center = chars(cx, cy);
    let bottom_left = chars(cx-1, cy+1);
    let bottom_right = chars(cx+1, cy+1);

    let diag1 = [top_left, center, bottom_right];
    let diag2 = [top_right, center, bottom_left];

    is_mas_or_sam(&diag1) && is_mas_or_sam(&diag2)
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
