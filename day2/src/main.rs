use std::fs;

fn get_levels_from(file_path: &str) -> Vec<Vec<i32>> {
    let level_str = fs::read_to_string(file_path).expect("Error reading file");
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in level_str.split('\n') {
        let mut curr_level: Vec<i32> = vec![];
        for v in line.split(' ') {
            if let Ok(v) = v.parse() {
                curr_level.push(v);
            }
        }
        if curr_level.len() > 0 {
            grid.push(curr_level);
        }
    }
    return grid;
}

fn is_safe(level: &Vec<i32>) -> bool{
    if level.len() <= 1 {
        return true;
    }
    
    let mut incline: i32 = 0;
    
    for i in 1..level.len() {
        let diff: i32 = level[i] - level[i-1];
        if diff.abs() < 1 || diff.abs() > 3 { 
            return false;
        } 
        if incline == 0 {
            if diff > 0 {
                incline = 1;
            } else if diff < 0 {
                incline = 2;
            }
        } else if incline == 1 && diff < 0 {
            return false;
        } else if incline == 2 && diff > 0 {
            return false;
        }
    }
    return true;
}


fn is_safe_v2(level: &Vec<i32>) -> bool{
    for i in 0..level.len() {
        let mut curr_vec: Vec<i32> = vec![]; 
        for j in 0..level.len() {
            if j != i{
                curr_vec.push(level[j]);
            }
        }
        if is_safe(&curr_vec) {
            return true;
        }
    }
    return false;
}
fn main() {
    let grid = get_levels_from("./input.txt");
    let mut num_safe = 0;
    for level in grid.iter() {
        if is_safe_v2(level){
            num_safe += 1;
        }
    }
    println!("number of levels safe: {}/{}", num_safe, grid.len());
}
