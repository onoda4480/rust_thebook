fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    // 最大値は{}です
    println!("The largest number is {}", largest);
 assert_eq!(largest, 100);
 //スコープ外だが↑のassert_eq!はlargest, 100が正しいことを確認するたマクロである
}