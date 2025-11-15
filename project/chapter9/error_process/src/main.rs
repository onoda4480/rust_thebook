fn main() {
    pub struct Guess {
        value: u32,
    }
    
    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
        
        pub fn value(&self) -> u32 {
            self.value
        }
    }
    
    // ゲッターを使う例
    let guess = Guess::new(42);
    
    // ゲッターで値を取得
    let val = guess.value();
    println!("The value is: {}", val);  // 42
    
    // 比較にも使える
    if guess.value() > 50 {
        println!("High!");
    } else {
        println!("Low!");
    }
    
    // 計算にも使える
    let doubled = guess.value() * 2;
    println!("Doubled: {}", doubled);  // 84
}