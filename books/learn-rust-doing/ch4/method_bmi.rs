// 身長と体重を表すBody構造体
struct Body {
    height: f64, // 身長cm
    weight: f64, // 体重kg
}

// Body構造体のメソッドを定義
impl Body {
    // BMIを計算するメソッド
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }
    // 乖離率を計算するメソッド
    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

fn main() {
    let taro = Body {
        height: 160.0,
        weight: 70.0,
    };
    println!("{}のBMI: {:.2}", "太郎", taro.calc_bmi());
    println!("{}の乖離率: {:.1}%", "太郎", taro.calc_per());
}
