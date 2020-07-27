pub fn solution(n: usize, d: usize, b: i64, mut a: Vec<i64>) -> u64 {
    for i in &mut a {
        *i -= b;
    }
    for i in 0..a.len() {
        if a[i] < 0 {
            if rearrange_is_possible(i, d, &mut a) {
                rearrange(i, d, &mut a);
            }
        }
    }
    let mid = match n % 2 {
        0 => n / 2,
        _ => (n + 1) / 2,
    };
    let front = &a[..mid];
    let back = &a[mid..];
    let x1 = front.iter().filter(|i| **i < 0).fold(0, |n, _| n + 1);
    let x2 = back.iter().filter(|i| **i < 0).fold(0, |n, _| n + 1);
    return std::cmp::max(x1, x2);
}

fn rearrange(i: usize, mut d: usize, a: &mut Vec<i64>) -> &Vec<i64> {
    d *= std::cmp::min(i + 1, a.len() - 1 - i + 1);
    let n = a.len();
    while a[i] < 0 && d > 0 {
        let mut edges: Vec<usize> = Vec::new();
        if i > d && a[i - d] > 0 {
            edges.push(i - d);
        }
        if d < n - i && a[i + d] > 0 {
            edges.push(i + d);
        }
        for edge in edges {
            while a[i] < 0 {
                let count = std::cmp::min(a[i].abs(), a[edge]);
                a[i] += count;
                a[edge] -= count;
            }
        }
        d -= 1;
    }
    return a;
}

fn rearrange_is_possible(i: usize, mut d: usize, a: &mut Vec<i64>) -> bool {
    let mut x = a[i];
    d *= std::cmp::min(i + 1, a.len() - 1 - i + 1);
    let n = a.len();
    while x < 0 && d > 0 {
        let mut edges: Vec<usize> = Vec::new();
        if i > d && a[i - d] > 0 {
            edges.push(i - d);
        }
        if d < n - i && a[i + d] > 0 {
            edges.push(i + d);
        }
        for edge in edges {
            while x < 0 {
                let count = std::cmp::min(x.abs(), a[edge]);
                x += count;
            }
        }
        d -= 1;
    }
    return x == 0;
}

#[test]
fn test_curfew() {
    assert_eq!(solution(5, 1, 1, vec![1, 0, 0, 0, 4]), 1);
    assert_eq!(solution(6, 1, 2, vec![3, 8, 0, 1, 0, 0]), 2);
}
