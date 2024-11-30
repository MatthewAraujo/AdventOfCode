use std::{fs, collections::HashMap};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("file.txt").expect("Failed to read file");
    let lines = input.lines();

    let input = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part_numbers = vec![];
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    'next_line: for (row, line) in input.iter().enumerate() {
        let mut col1 = 0;
        let mut col2;
        while col1 < line.len() {
            while !line[col1].is_numeric() {
                col1 += 1;
                if col1 >= line.len() {
                    continue 'next_line;
                }
            }
            col2 = col1;
            while col2 < line.len() && line[col2].is_numeric() {
                col2 += 1;
            }
            // number found
            let n: String = line[col1..col2].iter().copied().collect();
            let n: u32 = n.parse().unwrap();
            // dbg!(n);

            // check if it has a symbol in the neighborhood
            let start_row = if row > 0 { row - 1 } else { 0 };
            let end_row = (row + 2).min(input.len());
            let start_col = if col1 > 0 { col1 - 1 } else { 0 };
            let end_col = (col2 + 1).min(line.len());

            'outer: for i in start_row..end_row {
                for j in start_col..end_col {
                    if !input[i][j].is_numeric() && input[i][j] != '.' {
                        if input[i][j] == '*' {
                            if let Some(parts) = gears.get_mut(&(i,j)) {
                                parts.push(n);
                            } else {
                                gears.insert((i, j), vec![n]);
                            }
                        }
                        // println!("found {n} in row {row} col {col1}");
                        part_numbers.push(n);
                        break 'outer;
                    }
                }
            }

            col1 = col2;
        }
    }

    let part1: u32 = part_numbers.iter().sum();
    dbg!(part1);

    let part2: u32 = gears.into_values().filter(|g| g.len() == 2).map(|g| g.into_iter().product::<u32>()).sum();
    dbg!(part2);
}
