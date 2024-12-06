use std::fs;
use regex::Regex;

fn mul_do_dont_regex(file : &str) -> i64 {
    let it = Regex::new(r"(do\(\)|mul\((?<l>[0-9]+),(?<r>[0-9]+)\)|don't\(\))").unwrap();
                        //r"(do\(\)|mul\((?<l>[0-9]+),(?<r>[0-9]+)\)|don't\(\))"
    let mut sum: i64 = 0;
    let mut multiply = true;

    for line in file.lines() {
        if line.is_empty() {
            break;
        }
        
        let hits = it.captures_iter(line);

        for hit in hits {

            let do_dont = hit.get(0).unwrap().as_str();
            
            if do_dont.starts_with("don") {
                multiply = false;
            }
            else if do_dont.starts_with("do") {
                multiply = true;
            }
            else if multiply {

                let l = hit["l"].parse::<i64>().unwrap();            
                let r = hit["r"].parse::<i64>().unwrap();           
            
                sum += l * r;
            }
        }
    }
    return sum;
}

fn main() {

    let file = fs::read_to_string("real.txt").unwrap();

    let sum = mul_do_dont_regex(&file);

    println!("{}", sum);
}

