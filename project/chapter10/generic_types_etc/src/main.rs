fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
    //引数xと戻り値に対してライフタイム引数'aを指定しましたが、引数yには指定していません。 
    //yのライフタイムはxや戻り値のライフタイムとは何の関係もないからです。
}