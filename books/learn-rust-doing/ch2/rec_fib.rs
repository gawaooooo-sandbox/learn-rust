// フィボナッチ数を求める関数
fn fib(n: i64) -> i64 {
    if n == 1 { return 0; } // 再帰の終了条件
    if n == 2 { return 1; } // 再帰の終了条件
    return fib(n - 1) + fib(n - 2); // 再帰呼び出し
}

fn main() {
    for i in 2..=20 {
        println!("{}項目: {}", i, fib(i));
    }
}
