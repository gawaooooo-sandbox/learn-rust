// BMIb判定
fn main() {
    let body = Body::new("田中", 163.0, 75.2);
    body.print_result();
    let body = Body::new("佐藤", 158.2, 55.0);
    body.print_result();
    let body = Body::new("中村", 174.2, 54.2);
    body.print_result();
}

// b判定用の構造体
struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}
impl BmiRange {
    // オブジェクトを生成するメソッド
    fn new(min: f64, max: f64, label: &str) -> Self {
        Self {
            min,
            max,
            label: label.to_string(),
        }
    }
    // 範囲内かテストする関数
    fn test(&self, v: f64) -> bool {
        self.min <= v && v < self.max
    }
}

// 身長と体重を表す構造体
struct Body {
    height: f64, // 身長cm
    weight: f64, // 体重kg
    name: String,
}
impl Body {
    // オブジェクトを生成して返す
    fn new(name: &str, height: f64, weight: f64) -> Self {
        Self {
            height,
            weight,
            name: name.to_string(),
        }
    }
    // BMIを求める
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }
    // 肥満度判定を表示する
    fn print_result(&self) {
        let bmi = self.calc_bmi();
        // 判定用のオブジェクトを配列で生成
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重(やせ)"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満(1度)"),
            BmiRange::new(30.0, 35.0, "肥満(2度)"),
            BmiRange::new(35.0, 40.0, "肥満(3度)"),
            BmiRange::new(40.0, 99.0, "肥満(4度)"),
        ];
        let mut result = String::from("不明");
        // 肥満度判定
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        // 結果表示
        println!("{}さんのBMI: {:.1}, 判定: {}", self.name, bmi, result);
    }
}
