/// [問題 16] 各位の数字の和
///
/// ２^15=32768である、各位の数字の和は3+2+7+6+8=26となる。
/// 2^1000の各位の和はいくつか。
///
/// [方針]
/// 2乗した結果、各位をベクタに記録する。ただしベクタと数値の桁は逆順になる。
fn main() {
    let mut digits = vec![2u8];
    for _ in 1..1000 {
        let mut carry = 0u8;
        let mut i = 0usize;
        while i < digits.len() {
            let num = digits[i] * 2 + carry;
            digits[i] = num % 10;
            carry = num / 10;
            i += 1;
        }
        if 0 < carry {
            digits.push(carry);
        }
    }
    println!("{}", digits.iter().cloned().map(|n| n as u32).sum::<u32>());
}
