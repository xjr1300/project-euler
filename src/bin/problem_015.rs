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

fn lattice_path(size: usize) -> u64 {
    // グリッドの頂点数分のベクタを準備
    let mut grid: Vec<Vec<u64>> = Vec::new();
    for _ in 0..=size {
        grid.push(vec![0u64; size + 1]);
    }
    grid[0][0] = 1u64;

    let mut queue: Vec<(usize, usize)> = vec![(0, 0)];
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current.0 < size {
            if !queue.contains(&(current.0 + 1, current.1)) {
                queue.push((current.0 + 1, current.1));
            }
            grid[current.0 + 1][current.1] += grid[current.0][current.1];
        }
        if current.1 < size {
            if !queue.contains(&(current.0, current.1 + 1)) {
                queue.push((current.0, current.1 + 1));
            }
            grid[current.0][current.1 + 1] += grid[current.0][current.1];
        }
    }

    grid[size][size]
}

fn main() {
    let mut result = 1u64;
    for i in 1..=20 {
        result *= 20 + i;
        result /= i;
    }
    println!("{}", result);
    println!("lattice = {}", lattice_path(20));
}
