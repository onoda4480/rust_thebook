#![allow(unused)]
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
    //上記は添字midを使ってスライスを2つに分割していますが、
    //これは2つのスライスが被らないので、 スライスの異なる部分を借用することは、根本的に大丈夫だが,
    //コンパイラが借用規則を正しく理解できないため、エラーになります。
}
