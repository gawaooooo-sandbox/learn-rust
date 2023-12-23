// 配列をシャッフルするのに必要な宣言
use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数字を持つ配列を作成
    let mut numbers = [0; 75];
    for i in 1..=75 {
        numbers[i - 1] = i;
    }

    // シャッフル
    numbers.shuffle(&mut rand::thread_rng());

    // カードを表示
    for y in 0..5 {
        for x in 0..5 {
            let index = y * 5 + x;
            if index == 12 {
                print!("  *,");
            } else {
                print!("{:3},", numbers[index]);
            }
        }
        println!("");
    }
}
