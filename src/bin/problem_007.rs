/// [問題 7] 10001番目の素数
///
/// 最初の6個の素数をリストすると、2、3、5、7、11、そして13であり、6番目の素数は13である。
/// 10001番目の素数を求めよ。

fn is_prime(n: i32, primes: &[i32]) -> bool {
    primes.iter().all(|p| n % p != 0)
}

fn main() {
    let mut primes: Vec<i32> = Vec::new();
    for n in 2.. {
        if is_prime(n, &primes) {
            primes.push(n);
            if primes.len() == 10_001 {
                println!("{}", primes.last().unwrap());
                break;
            }
        }
    }
}
