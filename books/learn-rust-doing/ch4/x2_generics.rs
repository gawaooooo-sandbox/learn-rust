// 値を2倍するジェネリクス関数
fn x2<T: std::ops::Add<Output = T> + Copy>(a: T) -> T {
    a + a
}

fn main() {
    println!("{}", x2(3));
    println!("{}", x2(3.0f64));
    println!("{}", x2::<u64>(3));
}
