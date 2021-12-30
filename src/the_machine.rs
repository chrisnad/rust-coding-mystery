use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn the_machine() {
    let machine_matrix = matrix_from_file("resources/2dGridOfParticles.txt");
    println!("{}", manhattan_distance_sum(machine_matrix));
}

fn manhattan_distance_sum(matrix: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'•' {
                sum += manhattan_distance_from_center((x as i32, y as i32));
            }
        }
    }
    sum
}

fn manhattan_distance_from_center((x, y): (i32, i32)) -> i32 {
    // Center Particle (X:51, y:26)
    let x_diff = 51 - x;
    let y_diff = 26 - y;
    x_diff.abs() + y_diff.abs()
}

fn matrix_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut matrix = Vec::new();
    buf.lines().for_each(|line| {
        let mut row = Vec::new();
        line.unwrap().chars().for_each(|c| {
            row.push(c);
        });
        matrix.push(row);
    });
    matrix
}
