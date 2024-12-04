use std::io;


fn read_arrays() -> (Vec<i32>, Vec<i32>) {
    let mut input = String::new();
    let mut a1 =Vec::new();
    let mut a2 =Vec::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break; 
        }
        let parts: Vec<i32> = trimmed
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if parts.len() == 2 {
            a1.push(parts[0]);
            a2.push(parts[1]);
           
        } else {
            println!("Each row must contain exactly two numbers. Please try again.");
        }
    }

    (a1,a2)
}

fn read_freq() -> (Vec<i32>, Vec<i32>) {
    let mut input = String::new();
    let mut a1 =vec![0; 100000];;
    let mut a2 =vec![0; 100000];;

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break; 
        }
        let parts: Vec<usize> = trimmed
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        if parts.len() == 2 {
            a1[parts[0]]+=1;
            a2[parts[1]]+=1;
           
        } else {
            println!("Each row must contain exactly two numbers. Please try again.");
        }
    }

    (a1,a2)
}

pub fn part1(){
    let mut a1:Vec<i32>;
    let mut a2:Vec<i32>;
 
    (a1,a2)=read_arrays();
    a1.sort();
    a2.sort();
    
    let s:i32 = a1.into_iter().zip(a2.into_iter()).map(|(x,y)|(x-y).abs()).sum();
    println!("{}",s);
}

pub fn part2(){
    let a1:Vec<i32>;
    let a2:Vec<i32>;
 
    (a1,a2)=read_freq();
    
    let s:i32 = a1.into_iter().zip(a2.into_iter()).enumerate().map(|(i,(x,y))|i as i32*x*y).sum();
    println!("{}",s);
}

