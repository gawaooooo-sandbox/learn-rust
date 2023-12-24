use std::env; // コマンドライン引数のため
use std::fs; // ファイルの読み込みのため

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるかどうか調べる
    if args.len() < 2 {
        println!("ファイル名を指定してください");
        return;
    }
    // (0から数えて)1番目の要素を得る
    let filename = &args[1];
    // ファイルを読んで表示する
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}
