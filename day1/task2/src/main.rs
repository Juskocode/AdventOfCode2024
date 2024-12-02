use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> io::Result<()>{
    
    let file = File::open("real.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut list_1 = Vec::new();
    let mut freq_2 = HashMap::new();
    

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&first), Some(&second)) = (parts.get(0), parts.get(1)) {
            list_1.push(first.parse::<i32>().unwrap());
            *freq_2.entry(second.parse::<i32>().unwrap()).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for i in 0..list_1.len() {
        if let Some(&freq) = freq_2.get(&list_1[i]) {
            total += list_1[i] * freq;
        }

    }

    println!("sum of total: {}", total);
    
    Ok(())
}
