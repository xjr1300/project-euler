/// [問題 15] ラティスパス
///
/// 2x2のグリッドにおいて、左上から開始して、右または下にのみ移動して、右下まで移動できたとき
/// 右下の角までのルートは6つある。
/// 20x20のグリッドにおいては、いくつルートがあるか。
///
/// [考え方]
/// 左上から右下まで移動するには、右方向へ20回、下方向へ20回移動する必要がある。
/// 右方向または左方向に20回移動するまで、いつでも右方向または左方向へ移動できる。
/// また、移動回数の合計は40回である。
/// これは、40回の移動の中で、右方向に20回移動する組み合わせ問題に変換できる。
///     下方向に20回移動する組み合わせも同様。
///     ただし、この変換は正方形のグリッドでしか適用できない。
/// よって、40回の移動のうち、20回の移動を取る組み合わせ問題である。

fn main() {
    let mut result = 1u64;
    for i in 1..=20 {
        result *= 20 + i;
        result /= i;
    }
    println!("{}", result);
}