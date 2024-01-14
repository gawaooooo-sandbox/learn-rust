// 素数を返す8ビットイテレータ
struct PrimeIterator {
    // 8ビット符号なし整数
    n: u8,
}
// メソッドを定義
impl PrimeIterator {
    fn new() -> Self {
        PrimeIterator { n: 1 }
    }
    // self.nが素数かどうか調べる
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
// イテレーターを実装
impl Iterator for PrimeIterator {
    // Item型をu8型に定義
    type Item = u8;
    // 次の要素を返す
    fn next(&mut self) -> Option<Self::Item> {
        // 素数を探して返す
        loop {
            // self.nをインクリメントする
            self.n += 1;
            // 8ビットを超える素数を調べない
            if std::u8::MAX == self.n {
                return None;
            }
            // self.nが素数ならSome(self.n)を返す
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}

fn main() {
    // 素数イテレータを作る
    let prime_iter = PrimeIterator::new();
    // 素数を表示する（u8の最大値255未満の素数をすべて列挙して表示
    for n in prime_iter {
        print!("{},", n);
    }
}
