use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some((program, inputs)) = args.split_first() {
        println!("Program Name: {:?}", program);
        println!("Here are your inputs: {:?}", inputs);

        let mut numbers: Vec<isize> = inputs.iter().map(|x| x.parse().unwrap()).collect();
        println!("Here they are as Integers: {:?}", numbers);

        numbers.sort_unstable();
        println!("Here they are as sorted Integers: {:?}", numbers);
    }
}

