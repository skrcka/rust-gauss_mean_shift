use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use core::f32::consts::E;
use core::f32::consts::PI;
use std::ops::Mul;
use std::ops::Add;
use std::sync::mpsc::channel;
use std::sync::{Mutex};


trait Distance {
    fn get_distance(&self, point: &Self) -> f64;
}

#[derive(Clone, PartialEq, Debug)]
struct Point {
    label: String,
    loc: Vec<f64>
}

impl Distance for Point {
    fn get_distance(&self, point: &Point) -> f64 {
        let mut dist = 0.0;
        for i in 0..self.loc.len()
        {
            let d = self.loc[i] - point.loc[i];
            dist += d * d;
        }
        dist.sqrt()
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point { label: self.label, loc: self.loc.iter().map(|v| v * rhs).collect() }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let size = rhs.loc.len();
        if size != self.loc.len() {
            panic!("Vectors not same size!");
        }
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(self.loc[i] + rhs.loc[i]);
        }

        Point { label: format!("{}+{}", self.label, rhs.label), loc: vec }
    }
}

type CentroidMutex = Mutex<Vec<Point>>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_gauss(dist: f64, dimension: usize, bandwidth: f32) -> f32 {
    let f_1 = 1.0/(bandwidth * (2.0*PI).sqrt()).powf(dimension as f32);
    let f_2 = E.powf(-0.5*(dist as f32/bandwidth).powf(2.0));
    f_1 * f_2
}

fn main() {
    let bandwidth = 2000.0;
    let radius = 2.0;
    let min_distance = 0.000001;

    let mut points = Vec::new();

    let centroids : CentroidMutex = Mutex::new(Vec::new());

    if let Ok(lines) = read_lines("./points.txt") {
        for line in lines.skip(1) {
            if let Ok(ip) = line {
                let label: String = ip.split(",").take(1).collect();
                let locs = ip.split(",").skip(1).map(|x| x.parse::<f64>().unwrap()).collect();
                points.push(Point{label: label, loc: locs});
            }
            else {
                panic!("Failed to read line!");
            }
        }
    }
    else {
        panic!("Failed to read file!");
    }

    let dimension = points[0].loc.len();
    let original = points.clone();

    points.par_iter_mut().for_each(|p| {
        let mut centroid = p.clone();
        loop {
            let mut change = false;
            for point in &original {
                if p == point {
                    continue;
                }
                let dist = point.get_distance(&centroid);
                if dist <= radius {
                    let gauss = get_gauss(dist, dimension, bandwidth);
                    let old_centroid = centroid.clone();
                    centroid = centroid.clone() + (point.clone() * gauss as f64).clone();
                    let change_by = old_centroid.get_distance(&centroid);
                    // println!("{}", change_by);
                    if change_by > min_distance {
                        change = true;
                    }
                }
            }
            if !change {
                break;
            }
        }
        centroids.lock().unwrap().push(centroid);
    });
    println!("{:?}", centroids.lock().unwrap());
}
