use std::io;
use std::fs;

fn day_1() {
    let filename = "data/d1.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let mut sum = 0;
    for v_str in contents.split_ascii_whitespace() {
        let mass: i32 = v_str.trim().parse().expect("expected integer");
        let mut fuel = mass / 3 - 2;
        let mut local_sum = 0;
        while fuel > 0 {
            local_sum += fuel;
            fuel = fuel / 3 - 2;
        }
        sum += local_sum;
        println!("{} {} {}", v_str, mass, local_sum);
    }

    println!("total {}", sum);
}

fn solve_d2_rep(a: usize, b:usize) -> usize {
    let filename = "data/d2.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut codes: Vec<usize> = Vec::new();
    for v_str in contents.split(',') {
        codes.push(v_str.trim().parse().unwrap());
    }

    //println!("{:?}", codes);

    codes[1] = a;
    codes[2] = b;


    let mut i = 0;
    while i < codes.len() {
        match codes[i] as usize {
            1 => {
                let dest = codes[i+3];
                codes[dest] = codes[codes[i+1]]+codes[codes[i+2]];
            },
            2 => {
                let dest = codes[i+3];
                codes[dest] = codes[codes[i+1]]*codes[codes[i+2]];
            },
            99 => {
                i = codes.len();
            },
            _ => {
                panic!("bad opcode!");
            }
        }
        i += 4;
    }


    codes[0]
}

fn day_2() {

    for i in 0..100 {
        for j in 0..100 {
            let v = solve_d2_rep(i, j);
            if v == 19690720 {
                println!("solved {} and {} becomes {}", i, j, i*100+j);
                return;
            }
        }
    }
}

fn main() {
    day_2();
}
