// フィボナッチ数列を返すイテレーター
struct FibIterator {
    a: usize, // 符号なし整数
    b: usize,
}
impl FibIterator {
    fn new() -> Self {
        FibIterator { a: 1, b: 1 }
    }
}
// イテレーターを実装
impl Iterator for FibIterator {
    // Item型をusize型に定義
    type Item = usize;
    // 次の要素を返す
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.a;
        self.a = self.b;
        self.b = temp + self.b;
        // // aとbを足してcに代入する
        // let c = self.a + self.b;
        // // aをbに、bをcに移す
        // self.a = self.b;
        // self.b = c;
        // Some(self.a)を返す
        return Some(self.a);
    }
}

fn main() {
    // forを使って結果を5個表示する
    let fib_iter = FibIterator::new();
    // emurateメソッドは値に何回目の繰り返しかを表すインデックスを付与するメソッド
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 {
            break;
        }
        print!("{},", n);
    }
    println!("");

    // takeを使う場合
    let fib_iter = FibIterator::new();
    // takeメソッドはイテレーターから最初のn個の要素を取り出すメソッド
    fib_iter.take(10).for_each(|n| print!("{},", n));
}
