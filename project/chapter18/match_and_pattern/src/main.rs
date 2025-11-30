fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            // 範囲内のidが見つかりました: {}
            //idの値をid_variableに束縛しているので、それを使用できます
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            // 別の範囲内のidが見つかりました
            //idの値は使用できません
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        }
    }
}
