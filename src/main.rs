use std::env;

fn calculate(binary: &String) -> u32 {
    let mut length = binary.len() as u32;
    let mut fa: Vec<u32> = Vec::new();

    for c in binary.chars() {
        let d = c.to_digit(10).unwrap();
        let mut r = d * d.pow(length);

        if r == 0 {
            r = d.pow(length);
        }

        let acc: u32 = r * d;

        fa.push(acc);

        if length != 0 {
            length -= 1;
        }
    }

    let fr: u32 = fa.iter().sum();

    fr
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("None binary was provided.");
    }

    let binary = args.get(1).unwrap();

    let value = calculate(binary);

    println!("{:?}", value);
}
