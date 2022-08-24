/// [問題 4] 最大の回文積
///
/// 左右どちらから読んでも同じ値になる数を回文数という。
/// 2桁の数の積で表される回文数のうち、最大の数値は9009=91×99である.
/// 3桁の数の積で表される回文数の最大値を求めよ.

fn is_palindrome(num: i32) -> bool {
    /*
    let str = format!("{}", num);
    let mut left: usize = 0;
    let mut right = str.len() - 1;

    while left < right {
        let l_char = str.get(left..=left).unwrap();
        let r_char = str.get(right..=right).unwrap();
        if l_char != r_char {
            break;
        }
        left += 1;
        right -= 1;
    }

    right <= left
    */
    let mut source = num;
    let mut reversed = 0;
    while 0 < source {
        reversed = reversed * 10 + source % 10;
        source /= 10;
    }

    num == reversed
}

fn main() {
    let mut largest = std::i32::MIN;

    for m in 100..1000 {
        for n in 100..1000 {
            let mul = m * n;
            if is_palindrome(mul) && largest < mul {
                largest = mul;
            }
        }
    }
    println!("{}", largest);
}
