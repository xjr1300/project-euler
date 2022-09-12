/// [問題 23] 不足した合計
///
/// 完全数は、約数の合計が正確にその数に等しい数である。
/// 例えば、28の約数の合計は 1 + 2 + 4 + 7 + 14 = 28 で、その28が完全数を意味する。
///
/// 約数の合計がn未満の場合、数nは不足と呼ばれ、この合計がnを超える場合、数nは豊富と呼ばれる。
///
/// 12は最も小さい豊富な数で 1 + 2 + 3 + 4 + 6 = 16、2つの豊富な数の合計として記述できる最小の数は24である。
/// 数学的分析により、28123より大きいすべての整数は、2つの豊富な数の合計として記述できることが示されている。
/// しかし、この上限は、2つの豊富な数の合計として表現できない最大の数が、この制限よりも小さいことがわかっているにもかかわらず、
/// 分析によってこれ以上減らすことはできない。
///
/// 2つの豊富な数の合計として書ききれないすべての正の整数の合計を求めなさい。
use std::collections::{HashMap, HashSet};

fn sum_of_divisors(n: u64) -> u64 {
    let mut bound = (n as f64).sqrt() as u64;
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
    let mut divisors: HashMap<u64, u64> = HashMap::new();
    for n in 1..=28123 {
        divisors.insert(n, sum_of_divisors(n));
    }
    let mut abundants: Vec<u64> = Vec::new();
    for n in 1..=28123 {
        if n < *divisors.get(&n).unwrap() {
            abundants.push(n);
        }
    }
    let mut abundant_sums: HashSet<u64> = HashSet::new();
    for (i, m) in abundants.iter().enumerate() {
        for n in abundants.iter().skip(i) {
            abundant_sums.insert(m + n);
        }
    }
    let mut sum = 0u64;
    for n in 1..=28123 {
        if !abundant_sums.contains(&n) {
            sum += n;
        }
    }

    println!("{}", sum);
}
