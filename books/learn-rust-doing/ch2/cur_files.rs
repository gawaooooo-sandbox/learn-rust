use std::fs;

fn main() {
    // ファイル一覧を取得
    let files = fs::read_dir(".").expect("不正なパス");
    for ent in files {
        // エントリーを得る
        let entry = ent.unwrap();
        // PathBufを得る
        let path = entry.path();
        // ファイル名を表示
        let fname = path.to_str().unwrap_or("不正なファイル名");
        println!("{}", fname);
    }
}
