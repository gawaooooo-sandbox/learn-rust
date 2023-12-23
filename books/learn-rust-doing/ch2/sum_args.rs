fn main() {
    // コマンドライン引数を得る
    let args = std::env::args();
    let mut total = 0.0;
    // コマンドライン引数を順に加算
    for (i, s) in args.enumerate() {
        // 最初の引数はプログラム名なので無視
        if i == 0 {
            continue;
        }
        // コマンドライン引数を数値に変換
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    // 結果を表示
    println!("total: {}", total);
}
