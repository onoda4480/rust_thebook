extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
//Cargoはtestsディレクトリを特別に扱い、
//cargo testを走らせた時にのみこのディレクトリのファイルをコンパイルする
//cargo testに--test引数、 その後にファイル名を指定することで、
//特定のテストファイルだけを実行できる
//今回の場合だとcargo test --test integration_test
//でこのファイルだけを実行できる