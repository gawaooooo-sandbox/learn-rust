// Rustでシーザー暗号に関する関数
fn encrypt(text: &str, shift: i16) -> String {
    // 'A'と'Z'の文字コードをi16型で取得
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 結果を代入する変数を用意
    let mut result = String::new();
    // 文字列を一文字ずつ処理する
    for ch in text.chars() {
        // 文字コードを取得
        let mut code = ch as i16;
        // 文字コードが'A'から'Z'の範囲の場合のみ処理を行う
        if code >= code_a && code <= code_z {
            // shift分だけずらす
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // 文字コードから文字列に変換してresultに追加
        result.push(char::from(code as u8));
    }
    return result;
}

fn main() {
    // 関数を呼び出す
    let enc = encrypt("I LOVE YOU.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}
