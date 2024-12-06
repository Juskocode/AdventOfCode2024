use std::{fs, vec};


fn bfs(i : i32, j : i32, grid: &Vec<&[u8]>) -> i32 {
    let n = grid.len() as i32;
    let m = grid.first().unwrap().len() as i32;

    let mut count = 0;
    let w : &[u8]= b"XMAS";

    let dirs = [(1, 0), (0, 1), (1, 1), (-1, 1)];

    for dir in dirs {
        for sign in [-1, 1] {
            let nx = dir.0 * sign;
            let ny = dir.1 * sign;

            for pos in 0..4 {
                let x = i + pos * nx;
                let y = j + pos * ny;

                if x < 0 || y < 0 || x >= n || y >= m {
                    break;
                }

                if grid[x as usize][y as usize] != w[pos as usize] {
                    break;
                }

                if pos == 3 {
                    count+=1;
                }
            }
        }
    }
    return count;
}

fn count_words(grid : &Vec<&[u8]>) -> i32 {
    let n = grid.len();
    let m = grid.first().unwrap().len();

    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == b'X' {
                count += bfs(i as i32, j as i32, &grid);
            } 
        }
    }
    return count;
}

fn main() {
    
    let file = fs::read_to_string("input0.txt").unwrap();

    let mut grid: Vec<&[u8]> = vec![];

    for line in file.lines() {
        grid.push(line.as_bytes());
    }

    let count = count_words(&grid);

    println!("{}", count);
}
