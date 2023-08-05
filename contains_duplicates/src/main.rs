use std::collections::HashMap;

fn naive_duplicates(nums: &Vec<i32>) -> bool {
    if nums.len() < 2 {
        return false
    } 
    let vec_length = nums.len();

    for i in 0..(vec_length - 1) {
        for j in i+1..(vec_length) {
            if nums[i] == nums[j] {
                return true
            }
        }
    }
    false
}

fn optimal_duplicates(nums: &Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for n in nums {
        let counter = map.entry(*n).or_insert(0);
        *counter += 1;
        if counter > &mut 1 {
            return true
        }
    }

    let largest = map.values().max();

    match largest {
        None => {
            println!("Empty list provided!"); 
            return false;
        }
        Some(n) => {
            if n < &2 {
                return false
            } else {
                return true
            }

        }
    } 
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 1, 3, 2];
    
    let naive_result = naive_duplicates(&nums);
    println!("Naive implementation for {:?}: {}", nums, naive_result);


    let optimal_result = optimal_duplicates(&nums);
    println!("Hashmap implementation for {:?}: {}", nums, optimal_result);
}
