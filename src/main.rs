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

// fn solve_d2_rep(a: usize, b:usize) -> usize {
//     let filename = "data/d2.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut codes: Vec<usize> = Vec::new();
//     for v_str in contents.split(',') {
//         codes.push(v_str.trim().parse().unwrap());
//     }

//     //println!("{:?}", codes);

//     codes[1] = a;
//     codes[2] = b;


//     let mut i = 0;
//     while i < codes.len() {
//         match codes[i] as usize {
//             1 => {
//                 let dest = codes[i+3];
//                 codes[dest] = codes[codes[i+1]]+codes[codes[i+2]];
//             },
//             2 => {
//                 let dest = codes[i+3];
//                 codes[dest] = codes[codes[i+1]]*codes[codes[i+2]];
//             },
//             99 => {
//                 i = codes.len();
//             },
//             _ => {
//                 panic!("bad opcode!");
//             }
//         }
//         i += 4;
//     }


//     codes[0]
// }

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


// fn visit(c: &str, d: i32, hm:&HashMap<&str, Vec<&str>>) -> i32 {
//     let mut ct = d;
//     let kids = 
//     match hm.get(c) {
//         Some(kids) => {
//             for k in kids {
//                 ct += visit(k,d+1,hm);
//             }
//         },
//         None => {}
//     };
//     return ct
// }

// fn day_6() {
//     let filename = "data/d6.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut p_to_cs: HashMap<&str, Vec<&str>> = HashMap::new();
//     let mut c_to_p: HashMap<&str, &str> = HashMap::new();
//     for v_str in contents.split_ascii_whitespace() {
//         let pstrs: Vec<_> = v_str.trim().split(")").collect();
        
//         c_to_p.insert(pstrs[1], pstrs[0]);
//         match p_to_cs.get_mut(&pstrs[0]) {
//             Some(children) => {
//                 children.push(pstrs[1]);
//             },
//             None => {
//                 p_to_cs.insert(pstrs[0], vec![pstrs[1]]);
//             }
//         }
//     }


//     let top = {
//         let mut t = "COM";

//         while c_to_p.get(t).is_some() {
//             t = c_to_p.get(t).unwrap();
//         }
//         t
//     };

//     let mut count = 0;
//     println!("lol visit {}", visit(top, 0, &p_to_cs));

//     let dist = {
//         let mut s = "SAN";
//         let mut ps = Vec::new();
//         while c_to_p.get(s).is_some() {
//             s = c_to_p.get(s).unwrap();
//             ps.push(s);
//         }

//         let mut count_down = 0;
//         let mut y = "YOU";
//         while c_to_p.get(y).is_some() {
//             y = c_to_p.get(y).unwrap();
//             if ps.contains(&y) {
//                 break;
//             }
//             count_down += 1;
//         }

//         let mut s = "SAN";
//         while c_to_p.get(s).is_some() {
//             s = c_to_p.get(s).unwrap();
//             if y == s {
//                 break;
//             }
//             count_down += 1;
//         }
        
//         count_down
//     };

//     println!("dist: {}", dist);

//     //println!("top {} list {:?}", top, p_to_cs);
// }


// fn d7_load_codes() -> Vec<i32> {
//     let filename = "data/d7.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut codes: Vec<i32> = Vec::new();
//     for v_str in contents.split(',') {
//         codes.push(v_str.trim().parse().unwrap());
//     }

//     codes
// }

// fn day_7_check(inputs: &Vec<i32>, codes: &mut Vec<i32>, i: &mut usize) -> (i32, bool) {
//     let instr_size = {
//         let mut v = vec![4;100];
//         v[3] = 2;
//         v[4] = 2;
//         v[5] = 3;
//         v[6] = 3;
//         v[99] = 1;
//         v };

//     let mut input_idx = 0;
//     let mut output = inputs[inputs.len() - 1];

//     while *i < codes.len() {
//         let instr = codes[*i];
//         let opcode = instr % 100;
//         let mut params = Vec::new();
//         let mut div = 100;

//         for j in 1..instr_size[opcode as usize] {
//             params.push(if ((instr / div) % 10) != 0 { *i+j } else {codes[*i+j] as usize});
//             div = div * 10;
//         }

//         //println!("instr {} is opcode {} with params {:?}", instr, opcode, params);
//         //println!("vals are {} {} {}", codes[p0], codes[p1], codes[p2] );
//         match opcode as usize {
//             1 => {
//                 codes[params[2]] = codes[params[0]]+codes[params[1]];
//                 *i += 4;
//             },
//             2 => {
//                 codes[params[2]] = codes[params[0]]*codes[params[1]];
//                 *i += 4;
//             },
//             3 => {
//                 // put input at addr params[0];
//                 codes[params[0]] = if inputs.len() > input_idx {inputs[input_idx]} else { output };
//                 input_idx += 1;
//                 *i += 2;
//             },
//             4 => {
//                 // output at params[0]
//                 //output = codes[params[0]];
//                 *i += 2;
//                 return (codes[params[0]], false)
//             },
//             5 => {
//                 // jump if true
//                 if codes[params[0]] != 0 {
//                     *i = codes[params[1]] as usize;
//                 }
//                 else {
//                     *i += 3;
//                 }
//             }
//             6 => {
//                 // jump if false
//                 if codes[params[0]] == 0 {
//                     *i = codes[params[1]] as usize;
//                 }
//                 else {
//                     *i += 3;
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
//                 *i += 4;
//             },
//             8 => {
//                 // equal
//                 if codes[params[0]] == codes[params[1]] {
//                     codes[params[2]] = 1;
//                 }
//                 else {
//                     codes[params[2]] = 0;
//                 }
//                 *i += 4;
//             },
//             99 => {
//                 *i = codes.len();
//             },
//             _ => {
//                 panic!("bad opcode!");
//             }
//         }
//     }

//     return (output,true);
// }


// fn d7() {
//     let mut max = 0;
//     for i0 in 0..5 {
//         for i1 in 0..5 {
//             if i1 != i0 {
//             for i2 in 0..5 {
//                 if i2 != i0 && i2 != i1 {
//                 for i3 in 0..5 {
//                     if i3 != i2 && i3 != i1 && i3 != i0 {
//                         for i4 in 0..5 {
//                             if i4 != i3 && i4 != i2 && i4 != i1 && i4 != i0 {
//                                 let mut codes = vec![d7_load_codes(); 5];
//                                 let mut instr_ptr: Vec<usize> = vec![0; 5];
//                                 let mut prev = (0, false);
//                                 let order = vec![i0+5,i1+5,i2+5,i3+5,i4+5];
//                                 let mut kickoff = 0;
//                                 while prev.1 == false {
//                                     for i in 0..5 {
//                                         let args = if kickoff < 5 {vec![order[i],prev.0]} else {vec![prev.0]};
//                                         prev = day_7_check(&args, &mut codes[i], &mut instr_ptr[i]);
//                                         println!("output {} halt {}", prev.0, prev.1);
//                                         kickoff+=1;
//                                     }
//                                 }
                            
//                                 println!("order {:?} answer: {}", &order, prev.0);
//                                 if prev.0 > max {
//                                     max = prev.0;
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 }
//             }
//             }
//         }
//     }
//     println!("answer: {}", max);
// }

// fn day_8_a() {
//     let filename = "data/d8.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut layers = Vec::new();
//     let mut finallayer = vec![2; 25*6];
//     let mut layer = Vec::new();
//     let mut layerinfo = (0, 0, 0);
//     let mut bestlayerinfo = (10000,0,0);
//     for c in contents.chars() {
//         if c.is_digit(10) {
//             let d = c.to_digit(10).unwrap();
//             layer.push(d);
//             match d {
//                 0 => layerinfo.0 += 1,
//                 1 => layerinfo.1 += 1,
//                 2 => layerinfo.2 += 1,
//                 _ => {}
//             }
//             if layer.len() == (25 * 6) {
//                 println!("layer info {:?}", layerinfo);
//                 if layerinfo.0 < bestlayerinfo.0 {
//                     bestlayerinfo = layerinfo;
//                     println!("new best {:?}", layer);
//                 }
//                 layers.push(layer);
//                 layer = Vec::new();
//                 layerinfo = (0,0,0);
//             }
//         }
//     }

//     println!("best is {:?} val  {}", bestlayerinfo, bestlayerinfo.1 * bestlayerinfo.2);
// }

// fn day_8_b() {
//     let filename = "data/d8.txt";

//     let contents = fs::read_to_string(filename).unwrap();

//     let mut layers = Vec::new();
//     let mut finallayer = vec![2; 25*6];
//     let mut layer = Vec::new();
//     for c in contents.chars() {
//         if c.is_digit(10) {
//             let d = c.to_digit(10).unwrap();
//             if finallayer[layer.len()] == 2 { finallayer[layer.len()] = d; }
//             layer.push(d);

//             if layer.len() == (25 * 6) {
//                 layers.push(layer);
//                 layer = Vec::new();
//             }
//         }
//     }

//     for i in 0..6 {
//         println!("{:?}", finallayer.get((i*25+0)..(i*25+25)));
//     }
// }

// 
// 
// #[derive(Debug)]
// enum SpaceState {
//     Empty,
//     Asteroid
// }


// fn day_10_a() {
//     let mut space = {
//         let filename = "data/d10_test.txt";
//         println!("In file {}", filename);
    
//         let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");

//         let mut v = Vec::new();
//         for v_str in contents.split_ascii_whitespace() {
//             println!("{}", v_str);
//             let mut w = Vec::new();
//             for c in v_str.chars() {
//                 w.push( 
//                     match c {
//                         '#' => {SpaceState::Asteroid},
//                         _ => {SpaceState::Empty}
//                     }
//                 );
//             }
//             v.push(w);
//         }
//         v
//     };

//     //println!("{:?}", space);


//     let h = space.len();
//     let w = space[0].len();

//     let mut space_counts = vec![vec![0;w];h];

//     println!("h: {} w: {}", h, w);

//     let paths = {
//         let mut paths = Vec::new();
//         let mut visitedmask = vec![vec![false; w]; h];

//         for i in 1..(h as i32) {
//             for j in (i+1)..(w as i32) {
//                 let mut path0 = Vec::new();
//                 let mut path1 = Vec::new();
//                 let mut path2 = Vec::new();
//                 let mut path3 = Vec::new();
//                 let mut loci = i;
//                 let mut locj = j;
//                 while loci < h as i32 && locj < w as i32 && visitedmask[loci as usize][locj as usize] == false {
//                     path0.push((-loci, locj));
//                     path1.push((-loci, -locj));
//                     path2.push((-locj, loci));
//                     path3.push((-locj, -loci));
//                     visitedmask[loci as usize][locj as usize] = true;
//                     loci += i;
//                     locj += j;
//                 }
//                 if path0.len() > 0 {
//                     paths.push(path0);
//                     paths.push(path1);
//                     paths.push(path2);
//                     paths.push(path3);
//                 }
//             }
//         }

//         // special cases!
//         let mut path0 = Vec::new();
//         let mut path1 = Vec::new();
//         let mut path2 = Vec::new();
//         let mut path3 = Vec::new();
//         for k in 1..(h as i32) { // assuming h = w
//             path0.push((0,-k));
//             path1.push((-k,-k));
//             path2.push((-k,0));
//             path3.push((-k,k));
//         }
//         paths.push(path0);
//         paths.push(path1);
//         paths.push(path2);
//         paths.push(path3);

//         paths
//     };
//     println!("{:?}", paths);

//     for ph in 0..(space.len() as i32) {
//         let line = &space[ph as usize];
//         for pw in 0..(line.len() as i32) {
//             let point = &line[pw as usize];
//             match point {
//                 SpaceState::Asteroid => {
//                     for path in &paths {
//                         //println!("checking path {:?} for {} {}", path, ph, pw);
//                         for p in path {
//                             let xh = ph + p.0;
//                             let xw = pw + p.1;
//                             if xh < 0 || xh >= h as i32 || xw < 0 || xw >= w as i32 {
//                                 //println!("break neg/of {} {}", xh, xw);
//                                 break;
//                             }
        
//                             match space[xh as usize][xw as usize] {
//                                 SpaceState::Asteroid => {
//                                     space_counts[xh as usize][xw as usize] += 1;
//                                     space_counts[ph as usize][pw as usize] += 1;
//                                     //println!("hit pair {} {} and {} {}", xh, xw, ph, pw);
//                                     break;
//                                 },
//                                 _ => {}
//                             }
//                         }
//                     }
//                 },
//                 _ => {}
//             }
//         }
//     }
//     println!("counts {:?}", space_counts);

//     let mut max = 0;
//     for iy in 0..space_counts.len() {
//         let y = &space_counts[iy];
//         for iz in 0..y.len() {
//             let z= y[iz];
//             if z > max {
//                 max = z;
//                 println!("new max! {} is {} {}", max, iy, iz);
//             }
//         }
//     }

//     println!("max {}", max);
// }

// fn day_10_b() {

//     let mut space = {
//         let filename = "data/d10.txt";
//         println!("In file {}", filename);
    
//         let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");

//         let mut v = Vec::new();
//         for v_str in contents.split_ascii_whitespace() {
//             println!("{}", v_str);
//             let mut w = Vec::new();
//             for c in v_str.chars() {
//                 w.push( 
//                     match c {
//                         '#' => {SpaceState::Asteroid},
//                         _ => {SpaceState::Empty}
//                     }
//                 );
//             }
//             v.push(w);
//         }
//         v
//     };

//     //println!("{:?}", space);
//     let h = space.len();
//     let w = space[0].len();

//     let mut space_counts = vec![vec![0;w];h];

//     println!("h: {} w: {}", h, w);

//     let paths = {
//         let mut paths = Vec::new();
//         let mut visitedmask = vec![vec![false; w]; h];

//         for i in 1..(h as i32) {
//             for j in (i+1)..(w as i32) {
//                 let mut path0 = Vec::new();
//                 let mut path1 = Vec::new();
//                 let mut loci = i;
//                 let mut locj = j;
//                 while loci < h as i32 && locj < w as i32 && visitedmask[loci as usize][locj as usize] == false {
//                     path0.push((-loci, locj));
//                     path1.push((-locj, loci));
//                     visitedmask[loci as usize][locj as usize] = true;
//                     loci += i;
//                     locj += j;
//                 }
//                 if path0.len() > 0 {
//                     paths.push(path0);
//                     paths.push(path1);
//                 }
//             }
//         }
//         // special cases!
//         let mut path0 = Vec::new();
//         let mut path1 = Vec::new();
//         for k in 1..(h as i32) { // assuming h = w
//             path0.push((-k,-k));
//             path1.push((-k,0));
//         }
//         paths.push(path0);
//         paths.push(path1);

//         // now sort it
//         paths.sort_unstable_by(|p0, p1| {
//             let v0 = p0[0];
//             let v1 = p1[0];
//             let dv0 = v0.0 as f64 / ((v0.0*v0.0+v0.1*v0.1) as f64).sqrt().abs();
//             let dv1 = v1.0 as f64 / ((v1.0*v1.0+v1.1*v1.1) as f64).sqrt().abs();
//             dv0.partial_cmp(&dv1).unwrap()
//         });

//         paths
//     };
//     for p in &paths {
//         println!("{:?}", p[0]);
//     }
//     //println!("{:?}", paths);
//     // 
//     let bh = 29;
//     let bw = 26;

//     let mut num_zapped = 0;
//     while num_zapped <= 200 {
//         for q in 0..4 {
//             for path in &paths {
//                 //println!("checking path {:?} for {} {}", path, ph, pw);
//                 for p in path {
//                     let sp = match q {
//                         0 => {(bh + p.0, bw + p.1)},
//                         1 => {(bh + p.1, bw - p.0)},
//                         2 => {(bh - p.0, bw - p.1)},
//                         _ => {(bh - p.1, bw + p.0)}
//                     };

//                     let xh = sp.0;
//                     let xw = sp.1;
//                     if xh < 0 || xh >= h as i32 || xw < 0 || xw >= w as i32 {
//                         //println!("break neg/of {} {}", xh, xw);
//                         break;
//                     }
//                     let st = &space[xh as usize][xw as usize];
//                     match st {
//                         SpaceState::Asteroid => {
//                             space[xh as usize][xw as usize] = SpaceState::Empty;
//                             num_zapped += 1;
//                             println!("zapped asteroid {} at {} {}", num_zapped, xh, xw);
//                             break;
//                         },
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }
//     // zapped asteroid 200 at 19 14
// }


fn load_intcodes(filename: &str) -> HashMap<i64,i64> {
    let contents = fs::read_to_string(filename).unwrap();

    let mut codes: HashMap<i64,i64> = HashMap::new();
    let mut i = 0i64;
    for v_str in contents.split(',') {
        codes.insert(i,v_str.trim().parse().unwrap());
        i+=1;
    }

    codes
}

fn run_intcodes<F: FnMut(bool, i64) -> i64>(mut inout: F, codes: &mut HashMap<i64,i64>, i: &mut i64) -> bool {
    let instr_size = {
        let mut v = vec![4i64;100];
        v[3] = 2;
        v[4] = 2;
        v[5] = 3;
        v[6] = 3;
        v[9] = 2;
        v[99] = 1;
        v };

    let mut rel = 0i64;

    //let mut ip = 0i64;
    // while codes.contains_key(&ip) {
    //     let instr = codes[&ip];
    //     let opcode = instr % 100;
    //     print!("[{:5}]    ", ip);
    //     for i in 0..8 {
    //         print!("{:8} ", codes[&(ip+i)]);
    //     }
    //     print!("\n");
    //     ip += 8;
    // }

    while codes.contains_key(i) {
        let instr = codes[i];
        let opcode = instr % 100;
        let mut params = Vec::new();
        let mut div = 100;

        for j in 1..instr_size[opcode as usize] {
            params.push(
                match (instr / div) % 10 {
                    0 => codes[&(*i+j)],
                    1 => *i+j,
                    _ => codes[&(*i+j)]+rel,
                });
            div = div * 10;
        }

        let get_safe = |a| {if codes.contains_key(a) {codes[a]} else {0}};

        //print!("[{:5}]", i);
        //println!("ip {} instr {} is opcode {} with params {:?}", i, instr, opcode, params);
        //println!("vals are {} {} {}", codes[p0], codes[p1], codes[p2] );
        match opcode as usize {
            1 => {
                let p0 = get_safe(&params[0]);
                let p1 = get_safe(&params[1]);
                let o = codes.entry(params[2]).or_insert(0);
                *o = p0+p1;
                //println!("codes[{}] = {} + {}:  {}", params[2], p0, p1, *o);
                *i += 4;
            },
            2 => {
                let p0 = get_safe(&params[0]);
                let p1 = get_safe(&params[1]);
                let o = codes.entry(params[2]).or_insert(0);
                *o = p0*p1;
                //println!("codes[{}] = {} * {}:  {}", params[2], p0, p1, *o);
                *i += 4;
            },
            3 => {
                // put input at addr params[0];
                let o = codes.entry(params[0]).or_insert(0);
                *o = inout(false, 0i64);
                //println!("codes[{}] = input: {}", params[0], *o);
                *i += 2;
            },
            4 => {
                // output at params[0]
                //println!("output = codes[{}] ({})", params[0], get_safe(&params[0]));
                inout(true, get_safe(&params[0]));
                *i += 2;
                //return (get_safe(&params[0]), false)
            },
            5 => {
                // jump if true
                //println!("jmp to {} if {} != 0", get_safe(&params[1]), get_safe(&params[0]));
                if get_safe(&params[0]) != 0 {
                    *i = get_safe(&params[1]);
                }
                else {
                    *i += 3;
                }
            }
            6 => {
                // jump if false
                //println!("jmp to {} if 0 == {}", get_safe(&params[1]), get_safe(&params[0]));
                if get_safe(&params[0]) == 0 {
                    *i = get_safe(&params[1]);
                }
                else {
                    *i += 3;
                }
            },
            7 => {
                // less than
                // 
                //println!("codes[{}] = ({} < {})?1:0", params[2], get_safe(&params[0]), get_safe(&params[1]));
                if get_safe(&params[0]) < get_safe(&params[1]) {
                    let o = codes.entry(params[2]).or_insert(0);
                    *o = 1;
                }
                else {
                    let o = codes.entry(params[2]).or_insert(0);
                    *o = 0;
                }
                *i += 4;
            },
            8 => {
                // equal
                //println!("codes[{}] = ({} == {})?1:0", params[2], get_safe(&params[0]), get_safe(&params[1]));
                if get_safe(&params[0]) == get_safe(&params[1]) {
                    let o = codes.entry(params[2]).or_insert(0);
                    *o = 1;
                }
                else {
                    let o = codes.entry(params[2]).or_insert(0);
                    *o = 0;
                }
                *i += 4;
            },
            9 => {
                //println!("rel += {}   (rel before is {})", get_safe(&params[0]), rel);
                rel += get_safe(&params[0]);
                *i += 2;
            },
            99 => {
                //println!("exit");
                *i = -1;
            },
            _ => {
                panic!("bad opcode!");
            }
        }
    }

    return true;
}

#[derive(Debug)]
enum RobotDir {
    Up,
    Left,
    Right,
    Down
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct PanelCoord {
    x: i64,
    y: i64
}

enum RobotOutputStage {
    Paint,
    Move
}


fn day_11() {
    let mut codes = load_intcodes("data/d11.txt");

    let mut ip = 0i64;

    let mut rob_dir = RobotDir::Up;
    let mut rob_coords = PanelCoord { x: 0i64, y: 0i64 };

    let mut panel_info = HashMap::new();
    panel_info.insert(PanelCoord{x:0,y:0},1);

    let mut num_painted = 0;
    let mut output_stage = RobotOutputStage::Paint;

    // awkward way to capture a state because i can't figure out how to do a Fn and FnMut that reference the same state
    let d11_inout = |has_output, output| -> i64 {
        if has_output {
            match output_stage {
                RobotOutputStage::Paint => {
                    if panel_info.contains_key(&rob_coords) {
                        panel_info.entry(rob_coords).and_modify(|p| *p = output);
                    } else {
                        panel_info.insert(rob_coords, output);
                        num_painted += 1;
                        println!("num painted inc to {}", num_painted);
                    }
                    output_stage = RobotOutputStage::Move;
                    println!("got output {} for coords {:?}", output, &rob_coords);
                },
                _ => { //move
                    rob_dir = match (&rob_dir, output) {
                        (RobotDir::Up, 1) | (RobotDir::Down, 0) => { rob_coords.x += 1; RobotDir::Right },
                        (RobotDir::Up, 0) | (RobotDir::Down, 1) => { rob_coords.x -= 1; RobotDir::Left},
                        (RobotDir::Left, 1) | (RobotDir::Right, 0) => { rob_coords.y += 1; RobotDir::Up},
                        (RobotDir::Left, 0) | (RobotDir::Right, 1) => { rob_coords.y -= 1; RobotDir::Down},
                        _ => {
                            println!("Bad direction");
                            RobotDir::Up
                        }
                    };
                    println!("turned robot to {:?} and moved to {:?}", rob_dir, rob_coords);
                    output_stage = RobotOutputStage::Paint;
                }
            }
        }
        else {
            let input_color = if panel_info.contains_key(&rob_coords) {
                panel_info[&rob_coords]
            } else { 0i64 };
            println!("returning input {} from {:?}", input_color, &rob_coords);
            return input_color;
        }
        0
    };

    println!("running {}", run_intcodes(d11_inout, &mut codes,&mut ip));

    println!("num painted {}", num_painted);

    // hashmap to bitmap!
    let mut w = 0;
    let mut h = 0;
    for p in &panel_info {
        if p.0.x > w { w = p.0.x; }
        if (-1*p.0.y) > h { h = -1*p.0.y; };
    }

    println!("h {} w {}", h, w);

    for i in 0..=h {
        for j in 0..=w {
            let key = PanelCoord{x:j,y:(-1*i)};
            let mut v = " ";
            if panel_info.contains_key(&key) { if panel_info[&key] == 1i64 { v = "#"; } } 
            else { v = "?"; }

            print!("{}", v);
        }
        print!("\n");
    }
}

fn day_9_improved(mode: i64) {
    let mut codes = load_intcodes("data/d9.txt");

    let mut ip = 0i64;

    // awkward way to capture a state because i can't figure out how to do a Fn and FnMut that reference the same state
    let d9_inout = |has_output, output| -> i64 {
        if has_output {
            println!("output: {}", output);
        }
        else {
            return mode;
        }
        0
    };

    println!("running {}", run_intcodes(d9_inout, &mut codes,&mut ip));
}


fn main() {
    day_11();
}