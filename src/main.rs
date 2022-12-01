use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Point {
    label: String,
    loc: Vec<i32>
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut points = Vec::new();
    let dimension = 2;
    let n = 5;

    if let Ok(lines) = read_lines("./points.txt") {
        for (i, line) in lines.skip(1).enumerate() {
            if let Ok(ip) = line {
                println!("{}", i);
                let locs = ip.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
                points.push(Point{label: format!("test{i}").to_owned(), loc: locs});
            }
        }
    }

    points.par_iter_mut().for_each(|p| println!("{}", p.label));
}
