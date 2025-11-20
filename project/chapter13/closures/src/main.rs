use std::thread;
use std::hash::Hash;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        //    ^     ^
        //    │     └─ 戻り値の型
        //    └─ 引数の型（T ではない！）
        K: Eq + Hash + Copy,
        V: Copy,
{
    calculation: T,
    values: HashMap<K, V>,
    //      ^^^^^^^^^^^^
    //      ジェネリックにする（u32 ではない！）
}

impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        //    ^     ^
        //    struct と同じにする
        K: Eq + Hash + Copy,
        V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
