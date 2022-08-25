/// [問題 10] 素数の和
///
/// 10以下の素数の和は、2 + 3 + 5 + 7 = 17である。
/// 200万以下の素数の和を求めなさい。

// fn is_prime(n: u64, primes: &[u64]) -> bool {
//     primes.iter().cloned().all(|p| n % p != 0)
// }

fn eratosthenes(n: u64) -> Vec<u64> {
    // 0からnまでの数値が素数かを示すフラグを格納するベクタ
    let n = n as usize;
    let mut flags = vec![true; n + 1];
    // 0と1は素数でないためfalseを設定
    flags[0] = false;
    flags[1] = false;
    // nが素数かどうかは、ルートnまでの数値に対して、エラトステネスの篩にかければ良い
    let sqrt_n = (f64::sqrt(n as f64) + 0.1).trunc() as usize;
    for m in 2..=sqrt_n {
        if flags[m] {
            for mult in ((m * m)..=n).step_by(m) {
                flags[mult] = false;
            }
        }
    }
    let mut primes = Vec::new();
    for (i, v) in flags.iter().enumerate() {
        if *v {
            primes.push(i as u64);
        }
    }

    primes
}

fn main() {
    // let mut primes: Vec<u64> = Vec::new();
    // for n in 2..2_000_000 {
    //     if is_prime(n, &primes) {
    //         primes.push(n);
    //     }
    // }
    let primes = eratosthenes(2_000_000);
    println!("{}", primes.iter().sum::<u64>())
}
