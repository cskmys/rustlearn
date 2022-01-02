use std::collections::HashMap;

fn mean(v: &[i32]) -> f32 { // use &[i32] which is for slice which can be used with arrays, vectors etc
    // let mut sum = 0;
    // for i in v {
    //     sum += i;
    // }
    // sum as f32 / v.len() as f32
    v.iter().sum::<i32>() as f32 / v.len() as f32
}

fn median(v: &[i32]) -> i32 {
    let mut sorted_v = v.to_vec();
    sorted_v.sort();
    let mid = sorted_v.len()/2;
    sorted_v[mid]
}

fn mode(v: &[i32]) -> i32 {
    let mut f_table = HashMap::new();
    for &i in v { // using &i to use "i32" as keys instead of "&i32" as we can avoid borrow checker by not using references
        let cnt = f_table.entry(i).or_insert(0);
        *cnt += 1;
    }

    // let mut v: Vec<i32> = f_table.values().cloned().collect();
    // v.sort();
    // let max = v[v.len()-1];
    // let key = f_table.iter().find_map(|(&k, &val)| if val == max{ Some(k) } else { None });
    // match key {
    //     Some(&ret) => ret,
    //     None => -65535
    // }
    f_table.into_iter().max_by_key(|&(_, cnt)| cnt).map(|(val, _)| val).expect("booyah")
}

fn main() {
    let v = vec![4, 7, 2, 2, 2];

    println!("v: {:?}", v);
    println!("mean: {}, median: {}, mode: {}", mean(&v), median(&v), mode(&v));
}
