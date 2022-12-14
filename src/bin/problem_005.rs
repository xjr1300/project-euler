/// [問題 5] 最小の乗数
///
/// 2520は、1から10までの数値で余りなく割ることのできる、最小の数値である。
/// 1から20のすべての数値で完全に割り切れる最小の正の数値を求めよ。

fn main() {
    for n in 20.. {
        if (1..=20).all(|m| n % m == 0) {
            println!("{}", n);
            break;
        }
    }
}
