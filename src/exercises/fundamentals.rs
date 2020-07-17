/**
 * Finds count of trailing zeros in factorial of n
 */
pub fn zeros(n: u64) -> u64 {
  let mut total_of_2 = 0;
  let mut evens = (n - (n % 2)) / 2;
  while evens > 0 {
    total_of_2 += evens;
    evens = (evens - (evens % 2)) / 2;
  }
  let mut total_of_5 = 0;
  let mut multiples_of_5 = (n - (n % 5)) / 5;
  while multiples_of_5 > 0 {
    total_of_5 += multiples_of_5;
    multiples_of_5 =(multiples_of_5- (multiples_of_5% 5)) / 5;
  }
  return std::cmp::min(total_of_2, total_of_5);
}

#[test]
fn sample_tests() {
  assert_eq!(zeros(0), 0);
  assert_eq!(zeros(6), 1);
  assert_eq!(zeros(14), 2);
  assert_eq!(zeros(30), 7);
  assert_eq!(zeros(1000), 249);
  assert_eq!(zeros(100000), 24999);
  assert_eq!(zeros(1000000000), 249999998);
}
