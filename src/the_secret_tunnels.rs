use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn the_secret_tunnels() {
    let chars = chars_from_file("resources/ListOfInstructions.txt");
    let mut minimal_chars = clean_chars(&chars);
    while minimal_chars.len() != clean_chars(&minimal_chars).len() {
        minimal_chars = clean_chars(&minimal_chars);
    }

    let tunnel_matrix = matrix_from_file("resources/MapOfTheTunnels.txt");

    let mut pos = (3, 21);
    for c in chars {
        match c {
            'N' => if tunnel_matrix[pos.1 - 1][pos.0] != '█' {
                pos.1 -= 1;
            }
            'S' => if tunnel_matrix[pos.1 + 1][pos.0] != '█' {
                pos.1 += 1;
            }
            'E' => if tunnel_matrix[pos.1][pos.0 + 1] != '█' {
                pos.0 += 1;
            }
            'W' => if tunnel_matrix[pos.1][pos.0 - 1] != '█' {
                pos.0 -= 1;
            }
            _ => (),
        }
    }

    println!("({}, {})", pos.0, pos.1);
}

fn clean_chars(chars: &Vec<char>) -> Vec<char> {
    let mut cleaned_chars = Vec::new();
    let mut skip = false;
    chars.windows(2).for_each(|w| {
        if skip {
            skip = false;
        } else if chars_cancel_out(w[0], w[1]) {
            skip = true;
        } else {
            cleaned_chars.push(w[0]);
        }
    });
    if !skip {
        cleaned_chars.push(chars[chars.len() - 1]);
    }
    cleaned_chars
}

fn chars_cancel_out(char1:char, char2:char) -> bool {
    match [char1, char2] {
        ['N', 'S'] | ['S', 'N'] | ['E', 'W'] | ['W', 'E'] => true,
        _ => false,
    }
}

fn chars_from_file(filename: impl AsRef<Path>) -> Vec<char> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut chars = String::new();
    buf.lines().for_each(|line| {
        line.unwrap().chars().for_each(|c| {
            chars.push_str(c.to_string().as_str());
        });
    });
    let mut chars_vec = Vec::new();
    for c in chars.replace("\n", "").replace("\r", "").replace(" ", "").replace(",", "").chars() {
        chars_vec.push(c);
    }
    chars_vec
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
