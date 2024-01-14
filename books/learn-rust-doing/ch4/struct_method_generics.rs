// 構造体Pointを定義
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// メソッドを定義
impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    // コンストラクター
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // 値を加算
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    // Point型を生成
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    // 座標に値を加算
    pt.add(Point { x: 20, y: 30 });
    println!("{:?}", pt);
}
