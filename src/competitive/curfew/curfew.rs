pub fn solution(n: usize, d: usize, b: i64, mut a: Vec<i64>) -> u64 {
    for i in &mut a {
        *i -= b;
    }
    println!("Initial distribution: {:?}", a);
    for i in 0..a.len() {
        if a[i] < 0 {
            rearrange(i, d, &mut a);
            println!("After processing {} room array is:{:?}", i, a);
        }
    }
    let mid = match n % 2 {
        0 => n / 2,
        _ => (n + 1) / 2,
    };
    let front = &a[..mid];
    let back = &a[mid..];
    println!("front: {:?}", front);
    println!("back: {:?}", back);
    let x1 = front.iter().filter(|i| **i < 0).fold(0, |n, _| n + 1);
    let x2 = back.iter().filter(|i| **i < 0).fold(0, |n, _| n + 1);
    return std::cmp::max(x1, x2);
}

fn rearrange(i: usize, mut d: usize, a: &mut Vec<i64>) -> &Vec<i64> {
    d *= std::cmp::min(i + 1, a.len() -1 - i + 1);
    println!("Initial d={}", d);
    let n = a.len();
    while a[i] < 0 && d > 0 {
        println!("Current d={}", d);
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

#[test]
fn test_curfew() {
    assert_eq!(solution(5, 1, 1, vec![1, 0, 0, 0, 4]), 1);
    assert_eq!(solution(6, 1, 2, vec![3, 8, 0, 1, 0, 0]), 2);
}
