use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn find_freedom(n: usize, k: u64, nums: Vec<u64>) -> usize {
    let mut nums_set: BTreeSet<u64> = BTreeSet::from_iter(nums.into_iter());
    let mut freedom = 0;
    while !nums_set.is_empty() {
        let mut chain_len = 0;
        let num = nums_set.iter().next().unwrap();
        let mut num_cpy = *num;
        while nums_set.contains(&num_cpy) {
            nums_set.remove(&num_cpy);
            chain_len += 1;
            num_cpy = num_cpy * k;
        }
        match chain_len % 2 {
            0 => freedom += chain_len / 2,
            _ => freedom += (chain_len + 1) / 2,
        }
    }
    return freedom;
}

#[test]
fn test_find_freedom() {
    assert_eq!(find_freedom(6, 2, vec![2, 3, 6, 5, 4, 10]), 3);
}
