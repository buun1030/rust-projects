use log::Level;
use log::info;
use std::collections::HashMap;
type Score = f32;

fn main() {
    let mut scores: Vec<Score> = vec![5.6, 7.3, 2.1];
    //for (i, s) in scores.iter().enumerate() {
    for i in 0..scores.len() {
        scores[i] += 1.0;
    }
    println!("{:?}", scores);


    let mut diamond_graph = HashMap::new();

    diamond_graph.insert(1, vec![2, 4]);
    diamond_graph.insert(2, vec![1, 3, 4]);
    diamond_graph.insert(3, vec![2, 4]);
    diamond_graph.insert(4, vec![1, 2, 3]);

    let dg = &diamond_graph[&2];
    println!("{:?}", dg);

    let s = String::from("hello");
    //let slice = &s[0..2];
    let slice = &s[..2];
    println!("{:?}", slice);
}


//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// &mut T reference may be released to safe code
// need to add a lifetime annotation on every reference in the struct’s definition
// diamond_graph[&2]
// for i in 0..scores.len()
// Copy built-in -> false
// count = counts
// requird an irrefutable pattern
// lifetime are always inferred by hte compiler -> false
// fully-qualified
// zero-sized type
// 0 bits
// .ok()
// [i32; 10]
// c.encode_utf8
// Closures are represented by traits, which means you can’t return closures directly.
// Arc<Mutex<T>>