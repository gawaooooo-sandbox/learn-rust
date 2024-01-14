fn main() {
    // 1から7のうち奇数の値のみ出力する
    for i in 1..=7 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
