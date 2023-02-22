// run with `cargo run --bin section3`
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("vec: {:?}", vec);
    add_6_to_vec(&mut vec);
    println!("vec: {:?}", vec);
    vec.insert(0,0);
    println!("vec: {:?}", vec);
}

fn add_6_to_vec(vec: &mut Vec<i32>) {
    vec.push(6);
}