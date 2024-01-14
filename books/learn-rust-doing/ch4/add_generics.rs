// ジェネリクスを使ってaddを定義
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));

    // i32型のaddを使う（型を明示する
    let x = add::<i32>(10, 25);
    println!("{}", x);

    // f32型のaddを使う（型を明示する
    let y = add::<f32>(10.0, 25.0);
    println!("{}", y);

    // no implementation for `char + char`
    // println!("{}", add('a', 'a'));
}
