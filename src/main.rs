use std::env;

fn mean(values: &Vec<isize>) -> f32 {
    values.iter().sum::<isize>() as f32 / values.len() as f32
}

fn median(values: &mut Vec<isize>) -> f32 {
    values.sort_unstable();
    let mid = values.len() / 2;

    if (values.len() % 2) == 0 {
        (values[mid-1] + values[mid]) as f32 / 2.0
    } else {
        values[mid] as f32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some((program, inputs)) = args.split_first() {
        println!("Program Name: {:?}", program);
        println!("Here are your inputs: {:?}", inputs);

        let mut numbers: Vec<isize> = inputs.iter().map(|x| x.parse().unwrap()).collect();
        println!("Here they are as Integers: {:?}", numbers);

        println!("mean: {}", mean(&numbers));
        println!("median: {}", median(&mut numbers));
    }
}

