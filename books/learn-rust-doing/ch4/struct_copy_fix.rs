struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
    fn print(&self) {
        println!("{}({})", self.name, self.age);
    }
}

fn main() {
    // Taroを作成
    let taro = Person::new("Taro", 18);
    // JiroはTaroを複製して名前だけ変えたい
    let jiro = Person {
        name: "Jiro".to_string(),
        ..taro // 更新記法
    };
    // TaroとJiroを表示
    taro.print();
    jiro.print();
}
