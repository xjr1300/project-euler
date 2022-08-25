/// [問題 10] 素数の和
///
/// 10以下の素数の和は、2 + 3 + 5 + 7 = 17である。
/// 200万以下の素数の和を求めなさい。

fn is_prime(n: u64, primes: &[u64]) -> bool {
    primes.iter().cloned().all(|p| n % p != 0)
}

fn main() {
    let mut primes: Vec<u64> = Vec::new();
    for n in 2..2_000_000 {
        if is_prime(n, &primes) {
            primes.push(n);
        }
    }
    println!("{}", primes.iter().sum::<u64>())
}
