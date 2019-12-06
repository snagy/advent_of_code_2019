use std::fs;
use std::collections::HashMap;

// fn day_1() {
//     let filename = "data/d1.txt";
//     println!("In file {}", filename);

//     let contents = fs::read_to_string(filename)
//     .expect("Something went wrong reading the file");

//     let mut sum = 0;
//     for v_str in contents.split_ascii_whitespace() {
//         let mass: i32 = v_str.trim().parse().expect("expected integer");
//         let mut fuel = mass / 3 - 2;
//         let mut local_sum = 0;
//         while fuel > 0 {
//             local_sum += fuel;
//             fuel = fuel / 3 - 2;
//         }
//         sum += local_sum;
//         println!("{} {} {}", v_str, mass, local_sum);
//     }

//     println!("total {}", sum);
// }

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

// fn day_2() {

//     for i in 0..100 {
//         for j in 0..100 {
//             let v = solve_d2_rep(i, j);
//             if v == 19690720 {
//                 println!("solved {} and {} becomes {}", i, j, i*100+j);
//                 return;
//             }
//         }
//     }
// }

// fn d3_get_bounds(codes: &Vec<Vec<(&str, i32)>>) -> ((i32,i32),(i32,i32)) {
//     let mut bounds = ((0, 0), (0, 0));

//     let mut pos = (0, 0);
//     for code in codes {
//         for c in code {
//             match c.0 {
//                 "R" => {
//                     pos.0 += c.1;
//                     if pos.0 > (bounds.0).1 {
//                         (bounds.0).1 = pos.0;
//                     }
//                 },
//                 "L" => {
//                     pos.0 -= c.1; 
//                     if pos.0 < (bounds.0).0 {
//                         (bounds.0).0 = pos.0;
//                     }
//                 },
//                 "U" => {
//                     pos.1 += c.1;
//                     if pos.1 > (bounds.1).1 {
//                         (bounds.1).1 = pos.1;
//                     }
//                 },
//                 "D" => {
//                     pos.1 -= c.1;
//                     if pos.1 < (bounds.1).0 {
//                         (bounds.1).0 = pos.1;
//                     }
//                 }
//                 _ => {
//                     panic!("bad instruction");
//                 }
//             }
//         }
//         pos = (0, 0)
//     }

//     println!("bounds are {:?}", bounds);

//     bounds
// }

// fn day_3() {
//     let filename = "data/d3.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut codes = Vec::new();
    
//     for code_str in contents.split_ascii_whitespace() {
//         let mut code: Vec<(&str, i32)> = Vec::new();
//         for v_str in code_str.trim().split(',') {
//             let c = v_str.split_at(1); 
//             code.push((c.0,c.1.parse().unwrap()));
//         }
//         codes.push(code);
//     }

//     let bounds = d3_get_bounds(&codes);

//     let startpos = (-(bounds.0).0, -(bounds.1).0);
//     println!("bounds are {:?} startpos {:?}", bounds, startpos);
//     let mut pos = startpos;
//     let mut nearest = std::i32::MAX;

//     let mut grid: Vec<Vec<bool>> = vec![vec![false; ((bounds.1).1-(bounds.1).0 + 4) as usize]; ((bounds.0).1-(bounds.0).0 + 4) as usize];

//     for ic in 0..codes.len() {
//         let code = &codes[ic];
//         if ic == 0 {
//             for c in code {
//                 match c.0 {
//                     "R" => {
//                         for x in 1..=c.1 {
//                             grid[(pos.0+x) as usize][pos.1 as usize] = true;
//                         }
//                         pos.0 += c.1;
//                     },
//                     "L" => {
//                         for x in 1..=c.1 {
//                             grid[(pos.0-x) as usize][pos.1 as usize] = true;
//                         }
//                         pos.0 -= c.1;
//                     },
//                     "U" => {
//                         for y in 1..=c.1 {
//                             grid[pos.0 as usize][(pos.1+y) as usize] = true;
//                         }
//                         pos.1 += c.1;
//                     },
//                     "D" => {
//                         for y in 1..=c.1 {
//                             grid[pos.0 as usize][(pos.1-y) as usize] = true;
//                         }
//                         pos.1 -= c.1;
//                     }
//                     _ => {
//                         panic!("bad instruction");
//                     }
//                 }
//             }
//         } else {
//             for c in code {
//                 match c.0 {
//                     "R" => {
//                         for x in 1..=c.1 {
//                             if grid[(pos.0+x) as usize][pos.1 as usize] {
//                                 println!("Found intersection! {} {}", pos.0+x, pos.1);
//                                 let len = (startpos.0 - (pos.0+x)).abs()+(startpos.1 - pos.1).abs();
//                                 if len < nearest {
//                                     nearest = len;
//                                 }
//                             }
//                         }
//                         pos.0 += c.1;
//                     },
//                     "L" => {
//                         for x in 1..=c.1 {
//                             if grid[(pos.0-x) as usize][pos.1 as usize] {
//                                 println!("Found intersection! {} {}", pos.0-x, pos.1);
//                                 let len = (startpos.0 - (pos.0-x)).abs()+(startpos.1 - pos.1).abs();
//                                 if len < nearest {
//                                     nearest = len;
//                                 }
//                             }
//                         }
//                         pos.0 -= c.1;
//                     },
//                     "U" => {
//                         for y in 1..=c.1 {
//                             if grid[pos.0 as usize][(pos.1+y) as usize] {
//                                 println!("Found intersection! {} {}", pos.0, pos.1+y);
//                                 let len = (startpos.0 - (pos.0)).abs()+(startpos.1 - (pos.1+y)).abs();
//                                 if len < nearest {
//                                     nearest = len;
//                                 }
//                             }
//                         }
//                         pos.1 += c.1;
//                     },
//                     "D" => {
//                         for y in 1..=c.1 {
//                             if grid[pos.0 as usize][(pos.1-y) as usize] {
//                                 println!("Found intersection! {} {}", pos.0, pos.1-y);
//                                 let len = (startpos.0 - (pos.0)).abs()+(startpos.1 - (pos.1-y)).abs();
//                                 if len < nearest {
//                                     nearest = len;
//                                 }
//                             }
//                         }
//                         pos.1 -= c.1;
//                     }
//                     _ => {
//                         panic!("bad instruction");
//                     }
//                 }
//             }
//         }
//         pos = startpos;
//     }

//     println!("len is {}", nearest);
// }


// fn day_3_b() {
//     let filename = "data/d3.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut codes = Vec::new();
    
//     for code_str in contents.split_ascii_whitespace() {
//         let mut code: Vec<(&str, i32)> = Vec::new();
//         for v_str in code_str.trim().split(',') {
//             let c = v_str.split_at(1); 
//             code.push((c.0,c.1.parse().unwrap()));
//         }
//         codes.push(code);
//     }

//     let bounds = d3_get_bounds(&codes);

//     let startpos = (-(bounds.0).0, -(bounds.1).0);
//     println!("bounds are {:?} startpos {:?}", bounds, startpos);

//     let mut pos = startpos;
//     let mut nearest = std::i32::MAX;
//     let mut grid: Vec<Vec<i32>> = vec![vec![std::i32::MAX / 2; ((bounds.1).1-(bounds.1).0 + 4) as usize]; ((bounds.0).1-(bounds.0).0 + 4) as usize];

//     for ic in 0..codes.len() {
//         let code = &codes[ic];
//         let mut nsteps = 0;
//         if ic == 0 {
//             for c in code {
//                 match c.0 {
//                     "R" => {
//                         for x in 1..=c.1 {
//                             nsteps+=1;
//                             if grid[(pos.0+x) as usize][pos.1 as usize] > nsteps {
//                                 grid[(pos.0+x) as usize][pos.1 as usize] = nsteps;
//                             }
//                         }
//                         pos.0 += c.1;
//                     },
//                     "L" => {
//                         for x in 1..=c.1 {
//                             nsteps+=1;
//                             if grid[(pos.0-x) as usize][pos.1 as usize] > nsteps {
//                                 grid[(pos.0-x) as usize][pos.1 as usize] = nsteps;
//                             }
//                         }
//                         pos.0 -= c.1;
//                     },
//                     "U" => {
//                         for y in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[pos.0 as usize][(pos.1+y) as usize] > nsteps {
//                                 grid[pos.0 as usize][(pos.1+y) as usize] = nsteps;
//                             }
//                         }
//                         pos.1 += c.1;
//                     },
//                     "D" => {
//                         for y in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[pos.0 as usize][(pos.1-y) as usize] > nsteps {
//                                 grid[pos.0 as usize][(pos.1-y) as usize] = nsteps;
//                             }
//                         }
//                         pos.1 -= c.1;
//                     }
//                     _ => {
//                         panic!("bad instruction");
//                     }
//                 }
//             }
//         } else {
//             for c in code {
//                 match c.0 {
//                     "R" => {
//                         for x in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[(pos.0+x) as usize][pos.1 as usize] + nsteps < nearest {
//                                 println!("Found intersection! {} {}", pos.0+x, pos.1);
//                                 nearest = grid[(pos.0+x) as usize][pos.1 as usize] + nsteps;
//                             }
//                         }
//                         pos.0 += c.1;
//                     },
//                     "L" => {
//                         for x in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[(pos.0-x) as usize][pos.1 as usize] + nsteps < nearest {
//                                 println!("Found intersection! {} {}", pos.0-x, pos.1);
//                                 nearest = grid[(pos.0-x) as usize][pos.1 as usize] + nsteps;
//                             }
//                         }
//                         pos.0 -= c.1;
//                     },
//                     "U" => {
//                         for y in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[pos.0 as usize][(pos.1+y) as usize] + nsteps < nearest {
//                                 println!("Found intersection! {} {}", pos.0, pos.1+y);
//                                 nearest = grid[pos.0 as usize][(pos.1+y) as usize] + nsteps;
//                             }
//                         }
//                         pos.1 += c.1;
//                     },
//                     "D" => {
//                         for y in 1..=c.1 {
//                             nsteps += 1;
//                             if grid[pos.0 as usize][(pos.1-y) as usize] + nsteps < nearest {
//                                 println!("Found intersection! {} {}", pos.0, pos.1-y);
//                                 nearest = grid[pos.0 as usize][(pos.1-y) as usize] + nsteps;
//                             }
//                         }
//                         pos.1 -= c.1;
//                     }
//                     _ => {
//                         panic!("bad instruction");
//                     }
//                 }
//             }
//         }
//         pos = startpos;
//     }

//     println!("len is {}", nearest);
// }


// fn check_pass(pi: i32) -> bool {
//     let mut p = pi;
//     let mut pd = 10;
//     let mut n_match = 0;
//     let mut has_double = false;
//     for i in 0..6 {
//         let d = p % 10;
//         if d > pd {
//             return false;
//         }
//         else if d == pd {
//             n_match += 1;
//         }
//         else {
//             if n_match == 1 {
//                 has_double = true;
//             }
//             n_match = 0;
//             pd = d;
//         }
//         p /= 10;
//     }
//     return n_match == 1 || has_double;
// }

// fn day_4() {
//     let mut sum = 0;
//     for i in 134564..=585159 {
//         if check_pass(i) {
//             sum += 1;
//         }
//     }
//     println!("test 1 111111 is {}", check_pass(111111));
//     println!("test 2 223450 is {}", check_pass(223450));
//     println!("test 3 123789 is {}", check_pass(123789));
//     println!("test 3 335577 is {}", check_pass(335577));

//     println!("test 3 111122 is {}", check_pass(111122));
//     println!("test 3 112222 is {}", check_pass(112222));

//     println!("sum is {}", sum);
// }

// fn day_5() {
//     let filename = "data/d4.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut codes: Vec<i32> = Vec::new();
//     for v_str in contents.split(',') {
//         codes.push(v_str.trim().parse().unwrap());
//     }

//     //println!("{:?}", codes);

//     // codes[1] = a;
//     // codes[2] = b;

//     let instr_size = {
//         let mut v = vec![4;100];
//         v[3] = 2;
//         v[4] = 2;
//         v[5] = 3;
//         v[6] = 3;
//         v[99] = 1;
//         v };

//     let input = 5;
//     let mut output = 0;

//     let mut i = 0;
//     while i < codes.len() {
//         let instr = codes[i];
//         let opcode = instr % 100;
//         let mut params = Vec::new();
//         let mut div = 100;

//         for j in 1..instr_size[opcode as usize] {
//             params.push(if ((instr / div) % 10) != 0 { i+j } else {codes[i+j] as usize});
//             div = div * 10;
//         }

//         println!("instr {} is opcode {} with params {:?}", instr, opcode, params);
//         //println!("vals are {} {} {}", codes[p0], codes[p1], codes[p2] );
//         match opcode as usize {
//             1 => {
//                 codes[params[2]] = codes[params[0]]+codes[params[1]];
//                 i += 4;
//             },
//             2 => {
//                 codes[params[2]] = codes[params[0]]*codes[params[1]];
//                 i += 4;
//             },
//             3 => {
//                 // put input at addr params[0];
//                 codes[params[0]] = input;
//                 i += 2;
//             },
//             4 => {
//                 // output at params[0]
//                 output = codes[params[0]];
//                 i += 2;
//             },
//             5 => {
//                 // jump if true
//                 if codes[params[0]] != 0 {
//                     i = codes[params[1]] as usize;
//                 }
//                 else {
//                     i += 3;
//                 }
//             }
//             6 => {
//                 // jump if false
//                 if codes[params[0]] == 0 {
//                     i = codes[params[1]] as usize;
//                 }
//                 else {
//                     i += 3;
//                 }
//             },
//             7 => {
//                 // less than
//                 if codes[params[0]] < codes[params[1]] {
//                     codes[params[2]] = 1;
//                 }
//                 else {
//                     codes[params[2]] = 0;
//                 }
//                 i += 4;
//             },
//             8 => {
//                 // equal
//                 if codes[params[0]] == codes[params[1]] {
//                     codes[params[2]] = 1;
//                 }
//                 else {
//                     codes[params[2]] = 0;
//                 }
//                 i += 4;
//             },
//             99 => {
//                 i = codes.len();
//             },
//             _ => {
//                 panic!("bad opcode!");
//             }
//         }
//     }

//     println!("output {}", output);
// //    codes[0]
// }


fn visit(c: &str, d: i32, hm:&HashMap<&str, Vec<&str>>) -> i32 {
    let mut ct = d;
    let kids = 
    match hm.get(c) {
        Some(kids) => {
            for k in kids {
                ct += visit(k,d+1,hm);
            }
        },
        None => {}
    };
    return ct
}

fn day_6() {
    let filename = "data/d6.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut p_to_cs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut c_to_p: HashMap<&str, &str> = HashMap::new();
    for v_str in contents.split_ascii_whitespace() {
        let pstrs: Vec<_> = v_str.trim().split(")").collect();
        
        c_to_p.insert(pstrs[1], pstrs[0]);
        match p_to_cs.get_mut(&pstrs[0]) {
            Some(children) => {
                children.push(pstrs[1]);
            },
            None => {
                p_to_cs.insert(pstrs[0], vec![pstrs[1]]);
            }
        }
    }


    let top = {
        let mut t = "COM";

        while c_to_p.get(t).is_some() {
            t = c_to_p.get(t).unwrap();
        }
        t
    };

    let mut count = 0;
    println!("lol visit {}", visit(top, 0, &p_to_cs));

    let dist = {
        let mut s = "SAN";
        let mut ps = Vec::new();
        while c_to_p.get(s).is_some() {
            s = c_to_p.get(s).unwrap();
            ps.push(s);
        }

        let mut count_down = 0;
        let mut y = "YOU";
        while c_to_p.get(y).is_some() {
            y = c_to_p.get(y).unwrap();
            if ps.contains(&y) {
                break;
            }
            count_down += 1;
        }

        let mut s = "SAN";
        while c_to_p.get(s).is_some() {
            s = c_to_p.get(s).unwrap();
            if y == s {
                break;
            }
            count_down += 1;
        }
        
        count_down
    };



    println!("dist: {}", dist);

    //println!("top {} list {:?}", top, p_to_cs);
}

fn main() {
    day_6();
}
