struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
// コードを分解
// let sum: u32 = Counter::new().zip(Counter::new().skip(1))
//                              .map(|(a, b)| a * b)
//                              .filter(|x| x % 3 == 0)
//                              .sum();
// ステップ1: Counter を2つ作る
// Counter::new()              // 1, 2, 3, 4, 5
// Counter::new().skip(1)      // 2, 3, 4, 5
// //             ^^^^^^^
// //             最初の1つをスキップ
// ステップ2: zip() でペアにする
// Counter::new().zip(Counter::new().skip(1))
// zip() とは: 2つのイテレータを組み合わせてペアにする
// イテレータ1: 1, 2, 3, 4, 5
// イテレータ2: 2, 3, 4, 5

// zip の結果:
// (1, 2)
// (2, 3)
// (3, 4)
// (4, 5)
// なぜ5つじゃなくて4つ？
// 短い方に合わせる
// イテレータ2は4つしかないので、4ペア
// ステップ3: map() で掛け算
// .map(|(a, b)| a * b)
// 各ペアを掛け算:
// (1, 2) → 1 * 2 = 2
// (2, 3) → 2 * 3 = 6
// (3, 4) → 3 * 4 = 12
// (4, 5) → 4 * 5 = 20
// 結果: 2, 6, 12, 20
// ステップ4: filter() で3の倍数だけ
// .filter(|x| x % 3 == 0)
// 3の倍数だけ残す:
// 2  → 2 % 3 = 2  ❌ 除外
// 6  → 6 % 3 = 0  ✅ OK
// 12 → 12 % 3 = 0 ✅ OK
// 20 → 20 % 3 = 2 ❌ 除外
// 結果: 6, 12
// ステップ5: sum() で合計
// .sum()
// 合計:
// 6 + 12 = 18