use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()>{
    let file = File::open("real.txt")?;
    let reader = io::BufReader::new(file);


    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Some(&first), Some(&second)) = (parts.get(0), parts.get(1)) {
            list_1.push(first.parse::<i32>().unwrap());
            list_2.push(second.parse::<i32>().unwrap());

        }
    }

    list_1.sort();
    list_2.sort();
    
    let mut total_diff = 0;
    for i in 0..list_1.len() {
        total_diff += (list_1[i] - list_2[i]).abs();
    }

    println!("sum of total_diff: {}", total_diff);
    

    Ok(())
}
