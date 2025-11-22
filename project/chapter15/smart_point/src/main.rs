struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //参照外し型強制が無いと以下のように書く必要がある
    //hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}