/// [問題 11] グリッド内で最も大きな積
///
/// 以下の20x20のグリッドにおいて、斜めのラインの4つの数値が赤色でマークされている。
///
///     08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
///     49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
///     81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
///     52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
///     22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
///     24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
///     32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
///     67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
///     24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
///     21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
///     78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
///     16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
///     86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
///     19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
///     04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
///     88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
///     04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
///     20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
///     20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
///     01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
///
/// これらの数値の積は26 x 63 x 78 x 14 = 1788696である。
/// 20x20のグリッド内で、上、下、左、右または斜めの同じ方向の隣接する数値の最大の積を求めなさい。
///
/// [考え方]
/// 20x20のグリッドを一次元のベクタで表現する。
/// インデックスからグリッドの行と列のインデックスは以下の通り表現できる。
/// なお、行と列のインデックスは、0から19である。
///     行のインデックス = index / 20
///     列のインデックス = index % 20
/// つまりインデックスに対応する行と列のインデックスは、以下の通りである。
///     (index / 20, index % 20)
///
/// 数字の列がその方向に存在する条件は、以下の通りである。
///     上: 行のインデックスが3以上のとき。
///     右上: 行のインデックスが3以上のとき、かつ、列のインデックスが16以下のとき
///     右: 列のインデックスが16以下のとき
///     右下: 行のインデックスが16以下のとき、かつ、列のインデックスが16以下のとき
///     下: 行のインデックスが16以下のとき
///     左下: 行のインデックスが16以下のとき、かつ、列のインデックスが3以上のとき
///     左: 列のインデックスが3以上のとき
///     左上: 行のインデックスが3以上のとき、かつ、列のインデックスが3以上のとき

const GRID: &str = "\
    08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 \
    49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 \
    81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 \
    52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 \
    22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 \
    24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 \
    32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 \
    67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 \
    24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 \
    21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 \
    78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 \
    16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 \
    86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 \
    19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 \
    04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 \
    88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 \
    04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 \
    20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 \
    20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 \
    01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

// clock-wise order
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    URDiagonally,
    Right,
    DRDiagonally,
    Down,
    DLDiagonally,
    Left,
    ULDiagonally,
}

// get directions at position.
fn get_directions(pos: i32) -> Vec<Direction> {
    let row = pos / 20;
    let col = pos % 20;
    let mut directions = Vec::new();
    if 3 <= row {
        directions.push(Direction::Up);
        if col <= 16 {
            directions.push(Direction::URDiagonally);
        }
        if 3 <= col {
            directions.push(Direction::ULDiagonally);
        }
    }
    if row <= 16 {
        directions.push(Direction::Down);
        if col <= 16 {
            directions.push(Direction::DRDiagonally);
        }
        if 3 <= col {
            directions.push(Direction::DLDiagonally);
        }
    }
    if 3 <= col {
        directions.push(Direction::Left);
    }
    if col <= 16 {
        directions.push(Direction::Right);
    }

    directions
}

fn get_direction_mul(grid: &[i32], pos: i32, direction: Direction) -> i32 {
    let mut pos = pos;
    let addition = match direction {
        Direction::Up => -20,
        Direction::URDiagonally => -19,
        Direction::Right => 1,
        Direction::DRDiagonally => 21,
        Direction::Down => 20,
        Direction::DLDiagonally => 19,
        Direction::Left => -1,
        Direction::ULDiagonally => -21,
    };
    let mut result = grid[pos as usize];
    for _ in 0..3 {
        pos += addition;
        result *= grid[pos as usize];
    }

    result
}

fn main() {
    let grid: Vec<i32> = GRID
        .split(' ')
        .into_iter()
        .map(|g| g.parse::<i32>().unwrap())
        .collect();
    let mut largest = i32::MIN;
    for pos in 0..20 * 20 {
        let directions = get_directions(pos);
        for direction in directions {
            let mul = get_direction_mul(&grid, pos, direction);
            if largest < mul {
                largest = mul;
            }
        }
    }
    println!("{}", largest);
}

#[cfg(test)]
mod tests {
    use super::{get_directions, Direction};

    #[test]
    fn test_get_directions1() {
        for row in 0..3 {
            // 列インデックス0
            let directions = get_directions(20 * row + 0);
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            // 列インデックス2
            let directions = get_directions(20 * row + 2);
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            // 列インデックス3
            let directions = get_directions(20 * row + 3);
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            // 列インデックス16
            let directions = get_directions(20 * row + 16);
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            // 列インデックス17
            let directions = get_directions(20 * row + 17);
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            // 列インデックス19
            let directions = get_directions(20 * row + 19);
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
        }
    }

    #[test]
    fn test_get_directions2() {
        for row in 3..17 {
            // 列インデックス0
            let directions = get_directions(20 * row + 0);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            // 列インデックス2
            let directions = get_directions(20 * row + 2);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            // 列インデックス3
            let directions = get_directions(20 * row + 3);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス16
            let directions = get_directions(20 * row + 16);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::DRDiagonally));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス17
            let directions = get_directions(20 * row + 17);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス19
            let directions = get_directions(20 * row + 19);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::Down));
            assert!(directions.contains(&Direction::DLDiagonally));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
        }
    }

    #[test]
    fn test_get_directions3() {
        for row in 17..20 {
            // 列インデックス0
            let directions = get_directions(20 * row + 0);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            // 列インデックス2
            let directions = get_directions(20 * row + 2);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            // 列インデックス3
            let directions = get_directions(20 * row + 3);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス16
            let directions = get_directions(20 * row + 16);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::URDiagonally));
            assert!(directions.contains(&Direction::Right));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス17
            let directions = get_directions(20 * row + 17);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
            // 列インデックス19
            let directions = get_directions(20 * row + 19);
            assert!(directions.contains(&Direction::Up));
            assert!(directions.contains(&Direction::Left));
            assert!(directions.contains(&Direction::ULDiagonally));
        }
    }
}
