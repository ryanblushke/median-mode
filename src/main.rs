use std::env;
use std::collections::HashMap;

fn mean(values: &Vec<i32>) -> f32 {
    values.iter().sum::<i32>() as f32 / values.len() as f32
}

fn median(values: &mut Vec<i32>) -> f32 {
    values.sort_unstable();
    let mid = values.len() / 2;

    if (values.len() % 2) == 0 {
        (values[mid-1] + values[mid]) as f32 / 2.0
    } else {
        values[mid] as f32
    }
}

fn mode(values: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in values {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mut max_key = 0;
    let mut max_value = 0;

    for (k, v) in occurrences.iter() {
        if *v > max_value {
            max_value = *v;
            max_key = *k;
        }
    }

    max_key
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some((program, inputs)) = args.split_first() {
        println!("Program Name: {:?}", program);
        println!("Here are your inputs: {:?}", inputs);

        let mut numbers: Vec<i32> = inputs.iter().map(|x| x.parse().unwrap()).collect();
        println!("Here they are as Integers: {:?}", numbers);

        println!("mean: {}", mean(&numbers));
        println!("median: {}", median(&mut numbers));
        println!("mode: {}", mode(&numbers));
    }
}

