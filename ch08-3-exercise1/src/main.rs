extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn avg(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for val in v {
        sum += val;
    }

    sum as f64 / v.len() as f64
}

fn median(v: &mut Vec<i32>) -> f64 {
    v.sort();

    println!("Sorted vector: {:?}", v);

    let middle = v.len()/2;

    if v.len() % 2 == 0 {
        (v[middle-1] + v[middle]) as f64 / 2 as f64
    } else {
        v[middle] as f64
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for elem in v {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    println!("Map = {:?}", map);

    let mut key = 0;
    let mut value = 0;
    for (k, v) in map {
        if v > value {
            value = v;
            key = *k;
        }
    }

    key
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let len = 10;

    for _ in 0..len {
        let value = rand::thread_rng().gen_range(1, 11);
        v.push(value);
    }

    println!("Vector = {:?}", v);

    println!("Average = {}", avg(&v));

    println!("Median = {}", median(&mut v));

    println!("Mode = {}", mode(&v));
}
