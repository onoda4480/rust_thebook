pub fn setup() {
    // ここにライブラリテスト固有のコードが来る
    // setup code specific to your library's tests would go here
}
//この状態だとcommonモジュールを結合テストファイルとして扱ってしまう。
//commonモジュールはテストヘルパー関数を提供するためのものであり、
//テスト自体を含むべきではない。
//この問題を解決するには、tests/common.rsを作成する代わりに、
//tests/common/mod.rsを作成する必要がある。
