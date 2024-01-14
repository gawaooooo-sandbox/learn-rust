// 構造体Personの定義
struct Person {
    name: String,
    age: i32,
}
// Personのメソッドを定義
impl Person {
    // Persionを生成する関数を定義
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}

fn main() {
    // Personを生成
    let taro = Person::new("太郎".to_string(), 20);
    let jiro = Person::new("次郎".to_string(), 18);
    // フィールドを確認
    println!("{}: {}歳", taro.name, taro.age);
    println!("{}: {}歳", jiro.name, jiro.age);
}
