// add_i32関数を定義
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
// add_f32関数を定義
fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

fn main() {
    // i32型のadd_i32関数を使う
    let x = add_i32(10, 20);
    println!("{}", x);

    // f32型のadd_f32関数を使う
    let y = add_f32(10.0, 20.0);
    println!("{}", y);
}
