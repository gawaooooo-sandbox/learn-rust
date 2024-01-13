// 問題のあるプログラム
fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    show_message(g1); // 所有権をshow_messageに移動
    println!("{}", g1); // g1は使えない
}

fn show_message(msg: String) {
    println!("{}", msg);
}
