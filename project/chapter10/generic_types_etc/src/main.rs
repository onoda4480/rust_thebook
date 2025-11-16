fn largest<T: PartialOrd>(list: &[T]) -> &T {
//        ^^^^^^^^^^^^^^^              ^^
//        Copy 不要！                  参照を返す
    let mut largest = &list[0];  // 参照
    //                ^
    
    for item in list {  // item は &T（参照）
//      ^^^^            &item ではない
        if item > largest {
            largest = item;
        }
    }
    
    largest  // 参照を返す
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}