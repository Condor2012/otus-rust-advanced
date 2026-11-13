use std::collections::HashMap;

fn asd(n: String) {
    println!("{}", n);
}

fn main() {
    let words: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
    ];

    let fruits: Vec<String> = words.into_iter().collect();

    // words.iter() is an iterator over &String
    // words.into_iter() is an iterator over String
    // words.iter_mut() is an iterator over &mut String
    
    // for x in &words { } // = words.iter()
    // for x in &mut words { } // = words.iter_mut()
    // for x in words { } // = words.into_iter()

    let v: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
    let new_v: Vec<i32> = v.into_iter().filter(|&x| x % 2 == 0).collect(); // &i32
    
    println!("{:?}", new_v);

    println!("{:?}", fruits);

    let mut v = vec!["Alex".to_string(), "Yuriy".to_string(), "Alex".to_string()];

    for name in v.iter() {
        println!("{}", name);
    }

    for name in v.iter_mut() {
        name.push_str(" | ");
    }

    println!("{:?}", v);

    let v = vec!["Alex".to_string(), "Yuriy".to_string(), "Alex".to_string()];

    let new_v: Vec<String> = v.into_iter().map(|mut name| { name.push('!'); name }).collect();

    println!("{:?}", new_v);

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a: i32 = nums.iter().sum();
    let b: i32 = nums.clone().into_iter().sum();
    assert_eq!(a, b);

    dbg!("{}", nums);

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let n = "10".to_string();
    asd(n);
    // println!("{}", n);

    let c: Vec<i32> = nums.iter().copied().filter(|x| x % 2 == 0).collect();
    dbg!("{:?}", c);
    // dbg!("{:?}", nums);

    let d: Vec<i32> = nums.iter().map(|i: &i32| *i).collect();
    dbg!("{:?}", d);

    // let next_nums = nums.iter().fold(
    //     (Vec::new(), Vec::new()), 
    //     |(mut e, mut o), x| {
    //         if x % 2 == 0 { 
    //             e.push(x) 
    //         } else { 
    //             o.push(x) 
    //         } (e, o)}
    // );
    // 
    // dbg!("{:?} / {:?}", next_nums.0, next_nums.1);
    // 
    // enum Tokens {
    //     Word(String),
    //     Whitespace,
    //     Number(i32),
    //     Symbol(char),
    //     Eof,
    // }
    
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let counts: HashMap<&str, i32> = text.split_ascii_whitespace().fold(
        HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        }
    );
    println!("{:?}", counts);

    
}
