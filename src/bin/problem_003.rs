/// [問題 3] 最も大きい素因数
///
/// 13195の素因数は5、7、13と29である。
/// 600851475143の最も大きい素因数を求めよ。
///
/// Nが素数でないとき、2以上のある整数xが存在して、Nはｘで割り切れる。
/// ここで、xはsqrt(N)以下の整数である。

fn is_prime(n: u64, primes: &[u64]) -> bool {
    primes.iter().all(|&m| n % m != 0)
}

fn gen_primes(bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    for n in 2..=bound {
        if is_prime(n, &primes) {
            primes.push(n);
        }
    }

    primes
}

fn main() {
    let num = 600_851_475_143_f64;
    let bound = num.sqrt().trunc() as u64;
    let mut primes = gen_primes(bound);
    primes.reverse();
    let factor = primes.iter().find(|m| num as u64 % **m == 0).unwrap();

    println!("{}", factor)
}

#[cfg(test)]
mod tests {
    use super::gen_primes;

    #[test]
    fn test_gen_primes() {
        let primes = gen_primes(10);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }
}
