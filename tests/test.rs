use recursive::recursive;

#[recursive]
fn sum(nums: &[u64]) -> u64 {
    if let Some((head, tail)) = nums.split_first() {
        head + sum(tail)
    } else {
        0
    }
}

#[test]
fn test_sum() {
    let n = 10_000_000;
    let v: Vec<u64> = (0..n).collect();
    assert_eq!(sum(&v), 49999995000000);
}
