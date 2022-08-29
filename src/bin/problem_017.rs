/// [問題 17] 文字数のカウント
///
/// 1から5までの数字を単語でかけば、one, two, three, four, fiveで、全部で3 + 3 + 5 + 4 + 4 = 19文字使される。
/// もし、1から1000(one thousand)までのすべての数字を単語で書いた場合、何文字が使用されるか。
/// [注意]
/// スペースとハイフンはカウントしない。例えば、342(three hundred and forty-two)は23文字を含み、115(one hundred and fifteen)は20文字を含む。
/// 数値を出力するときの"and"のしようは英国の使用方法に準拠している。
use std::collections::HashMap;

use once_cell::sync::Lazy;

static NUMBERS: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "one");
    m.insert(2, "two");
    m.insert(3, "three");
    m.insert(4, "four");
    m.insert(5, "five");
    m.insert(6, "six");
    m.insert(7, "seven");
    m.insert(8, "eight");
    m.insert(9, "nine");
    m.insert(10, "ten");
    m.insert(11, "eleven");
    m.insert(12, "twelve");
    m.insert(13, "thirteen");
    m.insert(14, "fourteen");
    m.insert(15, "fifteen");
    m.insert(16, "sixteen");
    m.insert(17, "seventeen");
    m.insert(18, "eighteen");
    m.insert(19, "nineteen");
    m.insert(20, "twenty");
    m.insert(30, "thirty");
    m.insert(40, "forty");
    m.insert(50, "fifty");
    m.insert(60, "sixty");
    m.insert(70, "seventy");
    m.insert(80, "eighty");
    m.insert(90, "ninety");

    m
});

fn main() {
    let mut counter = 0usize;
    for n in 1u16..1_000 {
        print!("{}: ", n);
        if NUMBERS.contains_key(&n) {
            let text = NUMBERS.get(&n).unwrap();
            println!("{}", text);
            counter += text.len();
            continue;
        }
        let digit_1st = n % 10;
        let digit_2nd = n / 10 % 10;
        let digit_3rd = n / 100;

        // 100, 200, 300, ..., 900
        if 0 < digit_3rd {
            let s = NUMBERS.get(&digit_3rd).unwrap();
            print!("{} hundred", s);
            counter += s.len();
            counter += "hundred".len();
        }
        if digit_1st == 0 && digit_2nd == 0 {
            println!();
            continue;
        }
        if 0 < digit_3rd {
            print!(" and ");
            counter += "and".len();
        }

        // 2 digits
        let m = n % 100;
        if NUMBERS.contains_key(&m) {
            let text = NUMBERS.get(&m).unwrap();
            println!("{}", text);
            counter += text.len();
            continue;
        }
        let d2 = (m / 10) * 10; // 1の位を落とす
        let d1 = m % 10;
        let d2_str = NUMBERS.get(&d2).unwrap();
        let d1_str = NUMBERS.get(&d1).unwrap();
        println!("{}-{}", d2_str, d1_str);
        counter += d2_str.len();
        counter += d1_str.len();
    }
    println!("1000: one thousand");
    counter += "one".len();
    counter += "thousand".len();
    println!("{}", counter);
}
