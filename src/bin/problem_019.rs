/// [問題 19] 日曜日の数え上げ
///
/// 以下の情報があるが、自分自身のために何らかの調査を好むだろう。
///
/// * 1900年1月1日は月曜日
/// * 30日は9月、4月、6月そして11月である。
/// * 2月は28日あり、残りは31日である。うるう年のとき2月は29日ある。
/// * うるう年は西暦が4で割り切れる年だが、西暦が400で割り切れず100で割り切れる年はうるう年ではない。
///
/// 20世紀の間（1901年1月1日から2000年12月31日まで）に月の初めが日曜日になるのは何回か求めなさい。

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 400 != 0 && year % 100 == 0 {
            return false;
        }
        return true;
    }

    false
}

fn days_of_month(year: i32, month: i32) -> i32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => match is_leap_year(year) {
            true => 29,
            false => 28,
        },
        _ => 31,
    }
}

fn main() {
    let mut num_of_days = 0;
    let mut num_of_sunday = 0;
    for year in 1900..=2000 {
        for month in 1..=12 {
            num_of_days += days_of_month(year, month);
            if 1900 < year && num_of_days % 7 == 6 {
                num_of_sunday += 1;
            }
        }
    }
    println!("{}", num_of_sunday);
}
