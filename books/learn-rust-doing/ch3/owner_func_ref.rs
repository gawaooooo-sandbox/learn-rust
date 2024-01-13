fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    show_message(&g1); // g1の参照を渡す
    println!("{}", g1); // 所有権は移動していない
}

fn show_message(msg: &String) {
    println!("{}", msg);
}
