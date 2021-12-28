use std::{
    fs::File,
    fs::write,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::collections::HashMap;

pub fn shredded_papers() {

    let shredded_tuples = tuples_from_lines(lines_from_file("resources/ShreddedSheetOfPaper.txt"));
    let blank_tuples = tuples_from_lines(lines_from_file("resources/BlankSheetOfPaper.txt"));
    let map:HashMap<String, String> = shredded_tuples.into_iter().collect();

    let mut data = String::new();

    for tuple in blank_tuples {
        data += &*(map.get(&tuple.0).unwrap().to_owned() + "\n");
    }

    write("resources/Unshredded.txt", data).expect("Unable to write file");

}

// get tuples from lines
fn tuples_from_lines(lines: Vec<String>) -> Vec<(String, String)> {
    let mut tuples = Vec::new();
    for line in lines {
        let slice = line.chars().take(21).collect::<String>();
        tuples.push((slice, line));
    }
    tuples
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
