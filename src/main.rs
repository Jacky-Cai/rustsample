use std::collections::HashMap;
use std::mem;
use std::fs::File;

use rand::prelude::*;
use rand::thread_rng;
use std::io::Read;
use std::io;
use std::cmp::max;

mod fa;

struct Pair<T, E> {
    x: T,
    y: E,
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10i16);
    v.push(20i16);
    v
}

#[derive(Debug)]
struct User {
    email: String,
    username: String
}

fn main() {
    let path = "/tmp/xx.txt";
    let mut file = File::open(path);
    if let Ok(mut f) = file {
        let mut s = String::new();
        f.read_to_string(&mut s);
        println!("content: {} ", s);
    }

    let vv:Vec<i32> = (1..100).collect();
    let largest = vv.iter().fold(std::i32::MIN, |a,b| a.max(*b));
    println!("largest num is: {} ", largest);
    println!("smallest i32 is: {} ", std::i32::MIN);

    use crate::fa::print_aaa;
    crate::fa::bbb::print_bbb();
    print_aaa();

    let u1 = User {
        email: "hello".to_string(),
        username: "hello".to_string(),
    };

    let u2 = User{..u1};
    println!("u2.email={}, u2.username={}", u2.email, u2.username);
    println!("u2 = {:?}", u2);

    let mut map = HashMap::new();
    map.insert("who", 120);
    map.insert("are", 120);
    map.insert("you", 120);

    let mut  v: Vec<i32> = (1..100).collect();
    match  v.first() {
        Some(value) => println!("first is {}", value),
        None => println!("none")
    }
    println!("capacity: {}, size: {}", v.capacity(), v.len());
    v.reserve(1000);
    println!("capacity: {}, size: {}", v.capacity(), v.len());
    v.shrink_to_fit();
    v.push(100) ;
    v.push(101) ;
    v.sort_by(|a, b| b.cmp(a));
    println!("capacity: {}, size: {}", v.capacity(), v.len());
    println!("v is {:?}", v);

//    let ranNum = Rng::next_u32();

    let mut rng = thread_rng();
//    v.shuffle(rng);
//    v.shuffle(rng);
    v.shuffle(&mut rng);
    v.shuffle(&mut rng);
    v.shuffle(&mut rng);
    v.shuffle(&mut rng);
    println!("v is {:?}", v);


}

fn printMap(m: HashMap<&str, i32>) {
    for k in m.keys() {
        println!("key: {}, value {}", k, m[k]);
    }
    println!("capacity: {}, size: {}", m.capacity(), m.len())
}
