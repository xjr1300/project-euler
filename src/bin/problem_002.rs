/// [問題 2] フィボナッチ数の偶数
///
/// フィボナッチ数列において、それぞれ新しい項は前の2つの項を足すことによって生成される。
/// 1と2で開始される最初の10項はこのようになる。
///     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
/// フィボナッチ数列の項の値が400万以下のとき, 値が偶数の項の総和を求めよ.
fn main() {
    let mut before2 = 1; // フィボナッチ数列の2つ前の項
    let mut before1 = 2; // フィボナッチ数列の1つ前の項
    let mut sum = 2; // フィボナッチ数列の内、偶数の項の合計

    loop {
        let next_term = before2 + before1;
        if 4_000_000 < next_term {
            break;
        }
        if next_term % 2 == 0 {
            sum += next_term;
        }
        before2 = before1;
        before1 = next_term;
    }

    println!("{}", sum);
}
