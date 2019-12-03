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

fn d3_get_bounds(codes: &Vec<Vec<(&str, i32)>>) -> ((i32,i32),(i32,i32)) {
    let mut bounds = ((0, 0), (0, 0));

    let mut pos = (0, 0);
    for code in codes {
        for c in code {
            match c.0 {
                "R" => {
                    pos.0 += c.1;
                    if pos.0 > (bounds.0).1 {
                        (bounds.0).1 = pos.0;
                    }
                },
                "L" => {
                    pos.0 -= c.1; 
                    if pos.0 < (bounds.0).0 {
                        (bounds.0).0 = pos.0;
                    }
                },
                "U" => {
                    pos.1 += c.1;
                    if pos.1 > (bounds.1).1 {
                        (bounds.1).1 = pos.1;
                    }
                },
                "D" => {
                    pos.1 -= c.1;
                    if pos.1 < (bounds.1).0 {
                        (bounds.1).0 = pos.1;
                    }
                }
                _ => {
                    panic!("bad instruction");
                }
            }
        }
        pos = (0, 0)
    }

    println!("bounds are {:?}", bounds);

    bounds
}

fn day_3() {
    let filename = "data/d3.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut codes = Vec::new();
    
    for code_str in contents.split_ascii_whitespace() {
        let mut code: Vec<(&str, i32)> = Vec::new();
        for v_str in code_str.trim().split(',') {
            let c = v_str.split_at(1); 
            code.push((c.0,c.1.parse().unwrap()));
        }
        codes.push(code);
    }

    let bounds = d3_get_bounds(&codes);

    let startpos = (-(bounds.0).0, -(bounds.1).0);
    println!("bounds are {:?} startpos {:?}", bounds, startpos);
    let mut pos = startpos;
    let mut nearest = std::i32::MAX;

    let mut grid: Vec<Vec<bool>> = vec![vec![false; ((bounds.1).1-(bounds.1).0 + 4) as usize]; ((bounds.0).1-(bounds.0).0 + 4) as usize];

    for ic in 0..codes.len() {
        let code = &codes[ic];
        if ic == 0 {
            for c in code {
                match c.0 {
                    "R" => {
                        for x in 1..=c.1 {
                            grid[(pos.0+x) as usize][pos.1 as usize] = true;
                        }
                        pos.0 += c.1;
                    },
                    "L" => {
                        for x in 1..=c.1 {
                            grid[(pos.0-x) as usize][pos.1 as usize] = true;
                        }
                        pos.0 -= c.1;
                    },
                    "U" => {
                        for y in 1..=c.1 {
                            grid[pos.0 as usize][(pos.1+y) as usize] = true;
                        }
                        pos.1 += c.1;
                    },
                    "D" => {
                        for y in 1..=c.1 {
                            grid[pos.0 as usize][(pos.1-y) as usize] = true;
                        }
                        pos.1 -= c.1;
                    }
                    _ => {
                        panic!("bad instruction");
                    }
                }
            }
        } else {
            for c in code {
                match c.0 {
                    "R" => {
                        for x in 1..=c.1 {
                            if grid[(pos.0+x) as usize][pos.1 as usize] {
                                println!("Found intersection! {} {}", pos.0+x, pos.1);
                                let len = (startpos.0 - (pos.0+x)).abs()+(startpos.1 - pos.1).abs();
                                if len < nearest {
                                    nearest = len;
                                }
                            }
                        }
                        pos.0 += c.1;
                    },
                    "L" => {
                        for x in 1..=c.1 {
                            if grid[(pos.0-x) as usize][pos.1 as usize] {
                                println!("Found intersection! {} {}", pos.0-x, pos.1);
                                let len = (startpos.0 - (pos.0-x)).abs()+(startpos.1 - pos.1).abs();
                                if len < nearest {
                                    nearest = len;
                                }
                            }
                        }
                        pos.0 -= c.1;
                    },
                    "U" => {
                        for y in 1..=c.1 {
                            if grid[pos.0 as usize][(pos.1+y) as usize] {
                                println!("Found intersection! {} {}", pos.0, pos.1+y);
                                let len = (startpos.0 - (pos.0)).abs()+(startpos.1 - (pos.1+y)).abs();
                                if len < nearest {
                                    nearest = len;
                                }
                            }
                        }
                        pos.1 += c.1;
                    },
                    "D" => {
                        for y in 1..=c.1 {
                            if grid[pos.0 as usize][(pos.1-y) as usize] {
                                println!("Found intersection! {} {}", pos.0, pos.1-y);
                                let len = (startpos.0 - (pos.0)).abs()+(startpos.1 - (pos.1-y)).abs();
                                if len < nearest {
                                    nearest = len;
                                }
                            }
                        }
                        pos.1 -= c.1;
                    }
                    _ => {
                        panic!("bad instruction");
                    }
                }
            }
        }
        pos = startpos;
    }

    println!("len is {}", nearest);
}


fn day_3_b() {
    let filename = "data/d3.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut codes = Vec::new();
    
    for code_str in contents.split_ascii_whitespace() {
        let mut code: Vec<(&str, i32)> = Vec::new();
        for v_str in code_str.trim().split(',') {
            let c = v_str.split_at(1); 
            code.push((c.0,c.1.parse().unwrap()));
        }
        codes.push(code);
    }

    let bounds = d3_get_bounds(&codes);

    let startpos = (-(bounds.0).0, -(bounds.1).0);
    println!("bounds are {:?} startpos {:?}", bounds, startpos);

    let mut pos = startpos;
    let mut nearest = std::i32::MAX;
    let mut grid: Vec<Vec<i32>> = vec![vec![std::i32::MAX / 2; ((bounds.1).1-(bounds.1).0 + 4) as usize]; ((bounds.0).1-(bounds.0).0 + 4) as usize];

    for ic in 0..codes.len() {
        let code = &codes[ic];
        let mut nsteps = 0;
        if ic == 0 {
            for c in code {
                match c.0 {
                    "R" => {
                        for x in 1..=c.1 {
                            nsteps+=1;
                            if grid[(pos.0+x) as usize][pos.1 as usize] > nsteps {
                                grid[(pos.0+x) as usize][pos.1 as usize] = nsteps;
                            }
                        }
                        pos.0 += c.1;
                    },
                    "L" => {
                        for x in 1..=c.1 {
                            nsteps+=1;
                            if grid[(pos.0-x) as usize][pos.1 as usize] > nsteps {
                                grid[(pos.0-x) as usize][pos.1 as usize] = nsteps;
                            }
                        }
                        pos.0 -= c.1;
                    },
                    "U" => {
                        for y in 1..=c.1 {
                            nsteps += 1;
                            if grid[pos.0 as usize][(pos.1+y) as usize] > nsteps {
                                grid[pos.0 as usize][(pos.1+y) as usize] = nsteps;
                            }
                        }
                        pos.1 += c.1;
                    },
                    "D" => {
                        for y in 1..=c.1 {
                            nsteps += 1;
                            if grid[pos.0 as usize][(pos.1-y) as usize] > nsteps {
                                grid[pos.0 as usize][(pos.1-y) as usize] = nsteps;
                            }
                        }
                        pos.1 -= c.1;
                    }
                    _ => {
                        panic!("bad instruction");
                    }
                }
            }
        } else {
            for c in code {
                match c.0 {
                    "R" => {
                        for x in 1..=c.1 {
                            nsteps += 1;
                            if grid[(pos.0+x) as usize][pos.1 as usize] + nsteps < nearest {
                                println!("Found intersection! {} {}", pos.0+x, pos.1);
                                nearest = grid[(pos.0+x) as usize][pos.1 as usize] + nsteps;
                            }
                        }
                        pos.0 += c.1;
                    },
                    "L" => {
                        for x in 1..=c.1 {
                            nsteps += 1;
                            if grid[(pos.0-x) as usize][pos.1 as usize] + nsteps < nearest {
                                println!("Found intersection! {} {}", pos.0-x, pos.1);
                                nearest = grid[(pos.0-x) as usize][pos.1 as usize] + nsteps;
                            }
                        }
                        pos.0 -= c.1;
                    },
                    "U" => {
                        for y in 1..=c.1 {
                            nsteps += 1;
                            if grid[pos.0 as usize][(pos.1+y) as usize] + nsteps < nearest {
                                println!("Found intersection! {} {}", pos.0, pos.1+y);
                                nearest = grid[pos.0 as usize][(pos.1+y) as usize] + nsteps;
                            }
                        }
                        pos.1 += c.1;
                    },
                    "D" => {
                        for y in 1..=c.1 {
                            nsteps += 1;
                            if grid[pos.0 as usize][(pos.1-y) as usize] + nsteps < nearest {
                                println!("Found intersection! {} {}", pos.0, pos.1-y);
                                nearest = grid[pos.0 as usize][(pos.1-y) as usize] + nsteps;
                            }
                        }
                        pos.1 -= c.1;
                    }
                    _ => {
                        panic!("bad instruction");
                    }
                }
            }
        }
        pos = startpos;
    }

    println!("len is {}", nearest);
}


fn main() {
    day_3();
}
