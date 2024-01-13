// 身長と体重のデータを持つ構造体
struct Body {
    height: f64,
    weight: f64,
}

fn main() {
    // 構造体の初期化
    let ichiro = Body {
        height: 165.0,
        weight: 80.0,
    };
    let jiro = Body {
        height: 170.0,
        weight: 65.0,
    };

    // 関数を呼び出す
    println!("{}のBMI: {:.1}", "一郎", calc_bmi(&ichiro));
    println!("{}のBMI: {:.1}", "二郎", calc_bmi(&jiro));
}

// BMIを計算するだけの関数
fn calc_bmi(body: &Body) -> f64 {
    body.weight / (body.height / 100.0).powi(2)
}
