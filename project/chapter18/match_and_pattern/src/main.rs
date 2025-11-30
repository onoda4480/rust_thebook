fn main() {
    let num = Some(4);

    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
