use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    // いや〜！
    drop(v); // oh no!

    handle.join().unwrap();
}