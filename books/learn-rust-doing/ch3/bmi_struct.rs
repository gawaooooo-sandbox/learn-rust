// RustでBMI肥満度判定（判定表を利用）
// BMIの判定用構造体
struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str, // 判定ラベル
}

fn main() {
    // 身長と体重の入力
    let height_cm = input("身長(cm)を入力してください: ");
    let weight = input("体重(kg)を入力してください: ");
    // BMIの計算
    let height = height_cm / 100.0;
    let bmi = weight / height.powi(2);
    // 肥満度判定票をベクター型で用意
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "低体重(やせ)",
        },
        BmiRange {
            min: 18.5,
            max: 25.0,
            label: "普通体重",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "肥満(1度)",
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "肥満(2度)",
        },
        BmiRange {
            min: 35.0,
            max: 40.0,
            label: "肥満(3度)",
        },
        BmiRange {
            min: 40.0,
            max: 99.0,
            label: "肥満(4度)",
        },
    ];
    // 肥満度判定
    let mut result = "不明";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    // 結果表示
    println!("BMI={:.1}, 判定={}", bmi, result);
}

// 一行読み取ってf64で返す
fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    s.trim().parse().expect("数値変換エラー")
}
