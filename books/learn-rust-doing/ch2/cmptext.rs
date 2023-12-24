// ファイル操作のライブラリーを読む
use std::fs;

fn main() {
    // ファイル名を指定
    let afile = "../ch1/fizzbuzz_python.txt";
    let bfile = "../ch1/fizzbuzz_rust.txt";

    // ファイルを文字列として読む
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    // 念の為トリム
    let astr = astr.trim();
    let bstr = bstr.trim();

    // 両者を比較
    if astr == bstr {
        println!("OK");
    } else {
        println!("NG");
    }
}
