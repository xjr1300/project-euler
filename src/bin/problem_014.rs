/// [問題 14] 最長のコラッツ数列
///
/// 正の整数に以下の式で繰り返し生成される数列を定義する。
///     n -> n/2 (nが偶数)
///     n -> 3n + 1 (nが奇数)
/// 13からはじめるとこの数列は以下のようになる.
///     13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
/// 13から1まで10個の項になる. この数列はどのような数字からはじめても最終的には1になると考えられているが、まだそのことは証明されていない(コラッツ問題)。
/// 100万未満の数字の中で、どの数字からはじめれば最長の数列を生成するか.
/// 注意: 数列の途中で100万以上になってもよい
use std::time::Instant;

use project_euler::measure;

fn collats(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn problem() {
    let mut max_chain = 0;
    let mut max_chain_value = 0_u64;
    for n in 1u64..1_000_000 {
        let mut chain = 0;
        let mut m = n;
        while m != 1 {
            m = collats(m);
            chain += 1;
        }
        if max_chain < chain {
            max_chain = chain;
            max_chain_value = n;
        }
    }
    println!("{}", max_chain_value);
}

fn main() {
    measure!(problem());
}
