trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

// fn main() {
//     // 赤ちゃん犬は{}と呼ばれる
//     println!("A baby dog is called a {}", Dog::baby_name());
// }

// fn main() {
//     println!("A baby dog is called a {}", Animal::baby_name());
// } コンパイルエラー！

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
} //フルパスでDog内Animalトレイト用いることを明記
