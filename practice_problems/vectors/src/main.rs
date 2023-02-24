use std::collections::HashMap;


fn main() {
    let list = [1,1,1,2,2,4,5,6,7];
        
    med_mode(list);
}

fn med_mode (list: [i32; 9]) {
    let med = list[list.len() / 2];
    println!("Median: {med}");

    let mut scores = HashMap::new();
    for num in list {
        let count = scores.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;
    for (key, value) in scores {
        if value > max {
            max = value;
            mode = key;
        }
    }
    println!("Mode: {mode}")
}

