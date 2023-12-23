// Rustで素数を100個生成

// 素数判定をする関数
fn is_prime(n: usize) -> bool {
    // 2からn-1までの数で割り切れるかどうかを調べる
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    // 1とn自身以外で割り切れる数がなければ素数
    true
}

// 素数を100個求める関数
fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    // countが100になるまで繰り返す
    while count < 100 {
        // iが素数ならprimesに追加
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    // 素数を格納する配列
    let mut primes = [0; 100];
    // 素数を求める
    get_primes(&mut primes);
    // 素数を表示する
    println!("{:?}", primes);
}
