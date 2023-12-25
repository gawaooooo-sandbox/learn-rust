// 再帰的に呼び出す関数sumを定義
fn sum(n: i32) -> i32 {
    if n <= 0 { return 0; } // 再帰の終了条件
    return n + sum(n - 1); // 再帰呼び出し
}

fn main() {
    println!("1から10までの和は{}です。", sum(10));
}
