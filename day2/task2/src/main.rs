use std::fs;

fn is_safe(row : &Vec<i32>) -> i32 {
    
    let mut prev = row[1] - row[0];

    if prev == 0 || prev.abs() > 3 {
        return 0;
    }

    for i in 2..row.len() {
        let next = row[i] - row[i - 1];
        
        if prev * next < 0 || next == 0 || next.abs() > 3 {
            return 0;
        }
        prev = next;
    }
    return 1;
}

fn solve_1opt(row : &Vec<i32>) -> i32 {
    
    for i in 0..row.len() {
        let mut row_1opt = row.clone();
        row_1opt.remove(i);        

        if is_safe(&row_1opt) == 1 {
            return 1;
        }
    }
    return 0;
}

fn main() {

    let file = fs::read_to_string("real.txt").unwrap();

    let mut safe = 0;

    for line in file.lines() {
        if line.is_empty() {
            break;
        }
        
        let row : Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        //safe += is_safe(&row);
        safe += solve_1opt(&row);
    }
    
    println!("{}", safe);
}

