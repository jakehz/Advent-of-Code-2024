use std::fs;
use std::collections::HashMap;

fn get_lists_from(file_path: &str) -> Vec<Vec<u32>> {
    let list_str = fs::read_to_string(file_path).expect("Error reading file");
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    
    for line in list_str.split('\n') {
        let mut found_vals: Vec<u32> = vec![];
        for v in line.split(' ') {
            if let Ok(v) = v.parse() {
                found_vals.push(v);
            }
        }
        
        if found_vals.len() == 2{
            list1.push(found_vals[0]);
            list2.push(found_vals[1]);
        } 
    }
    
    return vec![list1, list2];
}

fn get_total_distance(lists: &Vec<Vec<u32>>) -> i32{
    let mut total: i32 = 0;
    
    
    for i in 0..lists[0].len(){
        let v1: i32 = lists[0][i] as i32;
        let v2: i32 = lists[1][i] as i32;
        total += (v1 - v2).abs();
    }
    return total;
}

fn get_similarity_score(lists: &Vec<Vec<u32>>) -> u32 {
    let mut num_count: HashMap<u32, u32> = HashMap::new();
    let mut similarity_score: u32 = 0;    
    for &v in lists[1].iter() {
        match num_count.get(&v) {
            Some(count) => {
                num_count.insert(v, count + 1);
            },
            None => {
                num_count.insert(v, 1);
            }
        }
    }
    for &v in lists[0].iter() {
        if let Some(count) = num_count.get(&v) {
            similarity_score += v * count;
        }
    }
    return similarity_score;
}

fn main() {
    let mut lists = get_lists_from("./input2.txt");
    assert!(lists.len() == 2);
    assert!(lists[0].len() == lists[1].len());
    lists[0].sort();
    lists[1].sort();
    let soln = get_total_distance(&lists);
    println!("Total distance{}", soln);

    let soln2 = get_similarity_score(&lists);
    println!("Similarity score: {}", soln2);

}
