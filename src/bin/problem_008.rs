/// 以下の1000桁の数値の中で、隣接する4つの数値の掛け算で最大の数値は9x9x8x9=5832である。
///     73167176531330624919225119674426574742355349194934
///     96983520312774506326239578318016984801869478851843
///     85861560789112949495459501737958331952853208805511
///     12540698747158523863050715693290963295227443043557
///     66896648950445244523161731856403098711121722383113
///     62229893423380308135336276614282806444486645238749
///     30358907296290491560440772390713810515859307960866
///     70172427121883998797908792274921901699720888093776
///     65727333001053367881220235421809751254540594752243
///     52584907711670556013604839586446706324415722155397
///     53697817977846174064955149290862569321978468622482
///     83972241375657056057490261407972968652414535100474
///     82166370484403199890008895243450658541227588666881      <- この行に`9989`がある。
///     16427171479924442928230863465674813919123162824586
///     17866458359124566529476545682848912883142607690042
///     24219022671055626321111109370544217506941658960408
///     07198403850962455444362981230987879927244284909188
///     84580156166097919133875499200524063689912560717606
///     05886116467109405077541002256983155200055935729725
///     71636269561882670428252483600823257530420752963450
/// 1000桁の数値の中で、最大乗数となる隣接する13の数値を見つけなさい。
/// そして、その乗数の数を求めなさい。

const DIGITS: &str = "\
    73167176531330624919225119674426574742355349194934\
    96983520312774506326239578318016984801869478851843\
    85861560789112949495459501737958331952853208805511\
    12540698747158523863050715693290963295227443043557\
    66896648950445244523161731856403098711121722383113\
    62229893423380308135336276614282806444486645238749\
    30358907296290491560440772390713810515859307960866\
    70172427121883998797908792274921901699720888093776\
    65727333001053367881220235421809751254540594752243\
    52584907711670556013604839586446706324415722155397\
    53697817977846174064955149290862569321978468622482\
    83972241375657056057490261407972968652414535100474\
    82166370484403199890008895243450658541227588666881\
    16427171479924442928230863465674813919123162824586\
    17866458359124566529476545682848912883142607690042\
    24219022671055626321111109370544217506941658960408\
    07198403850962455444362981230987879927244284909188\
    84580156166097919133875499200524063689912560717606\
    05886116467109405077541002256983155200055935729725\
    71636269561882670428252483600823257530420752963450\
";

fn nums_to_vec(nums: &str) -> Vec<u64> {
    nums.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

fn main() {
    dbg!(DIGITS);
    assert_eq!(DIGITS.len(), 1000);
    let mut largest = 0_u64;
    for pos in 0..=987 {
        let nums = DIGITS.get(pos..pos + 13).unwrap();
        println!("pos={}, nums={}", pos, nums);
        let v = nums_to_vec(nums);
        let mul = v
            .iter()
            .cloned()
            .reduce(|accum, item| accum * item)
            .unwrap();
        if largest < mul {
            largest = mul;
        }
    }
    println!("{}", largest);
}

#[cfg(test)]
mod tests {
    use super::nums_to_vec;

    #[test]
    fn test_num_to_vec() {
        let nums = "4803346934513";
        let v = nums_to_vec(nums);
        assert_eq!(v, vec![4, 8, 0, 3, 3, 4, 6, 9, 3, 4, 5, 1, 3]);
    }
}