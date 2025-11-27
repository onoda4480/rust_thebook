#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ];

    println!("=== パターン1: |Point {{ x, y }}| ===");
    // 自動参照外し
    points.iter().for_each(|Point { x, y }| {
        println!("x: {}, y: {}", x, y);
    });

    println!("\n=== パターン2: |&Point {{ x, y }}| ===");
    // 明示的な参照外し
    points.iter().for_each(|&Point { x, y }| {
        println!("x: {}, y: {}", x, y);
    });

    println!("\n=== パターン3: |p| ===");
    // 参照のまま
    points.iter().for_each(|p| {
        println!("x: {}, y: {}", p.x, p.y);
    });

    println!("\n=== どれも同じ結果！ ===");
}
