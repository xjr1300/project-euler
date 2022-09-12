/// [問題 31] 友好的な数字
///
/// d(n)をnの適切な約数の和と定義する（nよりも小さくて、nを割り切る数）。
/// もし、d(a) = b かつ d(b) = a で、aとbが異なる場合、aとbは友好的な数字であると呼ぶ。
///
/// 例えば、220の適切な約数は 1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110 なので d(220) = 284 である.
/// また、284の適切な約数は 1, 2, 4, 71, 142 なので d(284) = 220 である.
///
/// 10,000未満の友好的な数字の和を求めよ.
use std::collections::HashMap;

fn sum_of_divisors(n: u32) -> u32 {
    let mut bound = (n as f64).sqrt() as u32;
    let mut sum = 1;
    if 1 < n {
        if n == bound * bound {
            sum += bound;
            bound -= 1;
        }
        for m in 2..=bound {
            if n % m == 0 {
                sum += m + (n / m);
            }
        }
    }

    sum
}

fn main() {
    let mut divisors: HashMap<u32, u32> = HashMap::new();
    for n in 1..=10_000 {
        divisors.insert(n, sum_of_divisors(n));
    }

    let mut sum = 0u32;
    for a in 1..=10_000 {
        let b = *divisors.get(&a).unwrap();
        if let Some(b_sum) = divisors.get(&b) {
            if *b_sum <= 10_000 && *b_sum == a && a != b {
                println!("{}, {}", a, b);
                sum += a;
            }
        }
    }

    println!("{}", sum);
}
