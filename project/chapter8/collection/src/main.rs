fn main() {
    let v = vec![1, 2, 3, 4, 5];

    //&と[]を使ってベクタの要素にアクセスする方法
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    //getメソッドを使ってベクタの要素にアクセスする方法
    match v.get(2) {
        //                      "3つ目の要素は{}です"
        Some(third) => println!("The third element is {}", third),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }
}