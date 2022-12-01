use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use core::f32::consts::E;
use std::ops::Mul;
use std::ops::Add;


#[derive(Clone, PartialEq, Debug)]
struct Point {
    label: String,
    loc: Vec<f64>
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point { label: "mul".to_owned(), loc: self.loc.iter().map(|v| v * rhs).collect() }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let size = rhs.loc.len();
        if size != self.loc.len() {
            panic!("Vectors not same size");
        }
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(self.loc[i] + rhs.loc[i]);
        }

        Point { label: "add".to_owned(), loc: vec }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut points = Vec::new();
    let dimension = 2;
    //let n = 5;
    let radius = 2.0;

    if let Ok(lines) = read_lines("./points.txt") {
        for (i, line) in lines.skip(1).enumerate() {
            if let Ok(ip) = line {
                println!("{}", i);
                let locs = ip.split(",").map(|x| x.parse::<f64>().unwrap()).collect();
                points.push(Point{label: format!("test{i}").to_owned(), loc: locs});
            }
        }
    }

    let original = points.clone();

    points.par_iter_mut().for_each(|p| {
        let mut centroid = p.clone();
        println!("Before: {:?}", centroid);
        loop {
            let mut dist = 0.0;
            let mut change = false;
            for point in &original {
                for i in 0..dimension
                {
                    let d = point.loc[i] - centroid.loc[i];
                    dist += d * d;
                }
                if dist <= radius {
                    let upper = dist * dist;
                    let lower = 2.0 * radius * radius;
                    let sub = upper / lower;
                    let gauss = E.powf(-sub as f32);
                    centroid = centroid.clone() + (p.clone() * gauss as f64).clone();
                    change = true;
                    println!("New: {:?}", centroid);
                }
            }
            if !change {
                break;
            }
        }
        //println!("{}", original[0].label);
    });
}
