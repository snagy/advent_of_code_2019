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
fn main() {
    day_1();
}
