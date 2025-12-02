static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        // Rust 2024: 参照を作らず、値を直接読む
        let count = COUNTER;
        println!("COUNTER: {}", count);
    }
    //Rust 2024では、static mutへの参照作成がデフォルトで禁止された
    // Rust 2024以前のコード:
    // unsafe {
    //     let count = &COUNTER;
    //     println!("COUNTER: {}", *count);
    // }
}
