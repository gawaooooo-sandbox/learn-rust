fn main() {
    let mut g1 = String::from("穏やかな心は体に良い");
    g1 = show_message(g1); // 所有権をshow_messageに移動
    println!("{}", g1); // ok
}

// Stringを受け取り、Stringを返す関数
fn show_message(msg: String) -> String {
    println!("{}", msg);
    return msg; // msgを返す
}
