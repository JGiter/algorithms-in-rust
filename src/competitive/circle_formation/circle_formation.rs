use std::collections::VecDeque;

pub fn solution(n: usize, mut heights: Vec<i64>) -> u64 {
    heights.sort();
    let mut circle: VecDeque<i64> = VecDeque::<i64>::new();
    for i in 0..n - 1 {
        match i % 2 {
            0 => circle.push_back(heights[i]),
            _ => circle.push_front(heights[i])
        }
    }
    let mut max_discomfort: u64 = 0;
    for i in 0..n - 2 {
        let discomfort = (circle[i + 1] -circle[i]).abs();
        max_discomfort = std::cmp::max(discomfort as u64, max_discomfort);
    }
    return max_discomfort;
}

#[test]
fn test_circle() {
    assert_eq!(solution(5, vec![2, 1, 1, 3, 2]), 1);
    assert_eq!(solution(3, vec![30, 10, 20]), 10);
}
