
use std::{io::{BufReader, BufRead}, fs::File, collections::VecDeque};

use ndarray::{Array, Array2, Axis};

fn part1(field: &Array2<char>) -> u32{
    let filtered_field = filter_field(field);
    isolate_numbers(&filtered_field).into_iter().sum()
}

fn part2(field: &Array2<char>) -> u32 {

    let gear_indices = extract_gears(field);
    let gear_ratios = determine_gear_ratios(field, &gear_indices);

    return gear_ratios.iter().map(|(gear_one, gear_two)| gear_one * gear_two).sum();
}

fn extract_gears(field: &Array2<char>) -> Vec<(usize, usize)> {

    let mut res = Vec::new();

    for (index, &element) in field.indexed_iter() {
        if element == '*' {
            res.push(index);
        }
    }
    res    

}

fn determine_gear_ratios(field: &Array2<char>, gear_indices: &Vec<(usize, usize)>) -> Vec<(u32, u32)> {

    let mut res: Vec<(u32, u32)> = Vec::new();

    for (gear_row, gear_col) in gear_indices.iter() {
        let neighbour_indices = array_neighboring_indices(field, *gear_row, *gear_col);
        let mut used_indices = Vec::new();
        let mut numbers: Vec<u32> = Vec::new();

        for neighbour_index in neighbour_indices.iter() {
            if used_indices.contains(neighbour_index) || !field[[neighbour_index.0, neighbour_index.1]].is_digit(10) {
                continue;
            }

            let mut number_chars: VecDeque<char> = VecDeque::new();

            number_chars.push_front(field[[neighbour_index.0, neighbour_index.1]]);
            used_indices.push(*neighbour_index);

            let mut running_col = neighbour_index.1;

            while running_col > 0 {
                running_col -= 1;
                let e = field[[neighbour_index.0, running_col]];
                if e.is_digit(10) {
                    number_chars.push_front(e);
                    used_indices.push((neighbour_index.0, running_col));
                } else {
                    break;
                }
            }

            let mut running_col = neighbour_index.1;

            while running_col < field.dim().1 - 1 {
                running_col += 1;
                let e = field[[neighbour_index.0, running_col]];
                if e.is_digit(10) {
                    number_chars.push_back(e);
                    used_indices.push((neighbour_index.0, running_col));
                } else {
                    break;
                }
            }

            let s: String = number_chars.into_iter().collect();
            numbers.push(s.parse().unwrap());
        }

        if numbers.len() == 2 {
            res.push((numbers[0], numbers[1]));
        }
    }
    return res;
}

fn isolate_numbers(field: &Array2<char>) -> Vec<u32> {
    let mut clean_field: Array2<char> = field.clone();

    for element in clean_field.iter_mut(){
        if element == &'*' || element == &'.'{
            *element = ' '
        }
    }

    let mut res: Vec<u32> = Vec::new();

    for row in clean_field.axis_iter(Axis(0)) {
        let row_s: String = row.into_iter().collect();
        res.append(&mut row_s.split_whitespace().map(|s| s.parse().unwrap()).collect());
    }

    res
}

fn filter_field(field: &Array2<char>) -> Array2<char> {
    let mut filtered_field: Array2<char> = Array2::from_elem((field.shape()[0], field.shape()[1]), '.');

    let symbol_indices = extract_symbols(field);
    symbol_indices.iter().for_each(|(row, col)| filtered_field[[*row, *col]] = '*');

    let mut updated_indices = symbol_indices.clone();

    while !updated_indices.is_empty() {
        let neighbour_indices = updated_indices.iter().fold(Vec::new(), |mut res, (row, col)| {
            res.append(&mut array_neighboring_indices(field, *row, *col));
            res
        });
        updated_indices = neighbour_indices.into_iter().filter(|(row, col)| {
            filtered_field[[*row, *col]]=='.' && field[[*row, *col]].is_digit(10)
        }
        ).collect();
        updated_indices.iter().for_each(|(row, col)| filtered_field[[*row, *col]] = field[[*row, *col]]);
    }
    filtered_field
}

fn extract_symbols(field: &Array2<char>) -> Vec<(usize, usize)> {

    let mut res = Vec::new();

    for (index, &element) in field.indexed_iter() {
        if element != '.' && !element.is_digit(10) {
            res.push(index);
        }
    }
    res
}


// Function to get the neighbors of a specific element in a 2D array
fn array_neighboring_indices<T>(array: &Array2<T>, row: usize, col: usize) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::new();

    // Iterate over neighboring rows and columns
    for &i in &[row.wrapping_sub(1), row, row + 1] {
        for &j in &[col.wrapping_sub(1), col, col + 1] {
            // Check if the coordinates are within the array bounds
            if i < array.shape()[0] && j < array.shape()[1] {
                // Skip the center element (the original element itself)
                if i != row || j != col {
                    neighbors.push((i, j));
                }
            }
        }
    }

    neighbors
}


fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let rows = lines.len();
    let columns = lines[0].len();

    let field: Array2<char> = Array::from_shape_fn((rows, columns), |(row, column)| lines[row].chars().nth(column).unwrap());

    println!("part 1 answer {}", part1(&field));
    println!("part 2 answer {}", part2(&field));



}