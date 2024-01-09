// 問題のあるプログラム
fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    let g2 = g1; // 所有権をg2に移動
    println!("{}", g1); // g1は使えない
}
