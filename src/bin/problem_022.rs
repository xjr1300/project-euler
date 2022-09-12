/// [問題 22] 名前の点数
///
/// 5000個以上のファースト・ネームを含んでいる46Kのテキストファイルnames.txtを使用して、まずアルファベット順にソートしなさい。
/// そして、各名前についてアルファベットに値を割り振り、リスト中の出現順の数と掛け合わせることで、名前のスコアを計算する.
///
/// 例えば、リストがアルファベット順にソートされているとき、COLINはリストの938番目にあり、3 + 15 + 12 + 9 + 14 = 53 の価値がある。
/// よって、COLINは 938 × 53 = 49714 というスコアを持つ.
///
/// ファイル中のすべての名前のスコアの合計を求めよ。
use std::fs;

fn read_names() -> Vec<String> {
    let contents = fs::read_to_string("p022_names.txt").unwrap();

    contents
        .split(',')
        .map(|name| name[1..name.len() - 1].to_string())
        .collect()
}

fn name_score(name: &str) -> u64 {
    let mut score = 0u64;
    for ch in name.chars() {
        score += (ch as u64) - 64;
    }

    score
}

fn main() {
    let mut names = read_names();
    assert!(names.contains(&"MARY".to_string()));
    assert!(names.contains(&"ALONSO".to_string()));
    names.sort();
    let mut sum = 0u64;
    for (index, name) in names.iter().enumerate() {
        sum += (index + 1) as u64 * name_score(name);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::name_score;

    #[test]
    fn test_name_score() {
        assert_eq!(name_score("COLIN"), 53);
    }
}
