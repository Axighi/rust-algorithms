#[allow(dead_code)]
pub fn bottom_up_cut_rod(p: Vec<i32>, n: usize) -> i32 {
  let mut r = vec![0; n + 1];
  for j in 1..n + 1 {
    let mut q = i32::MIN;
    for i in 1..j + 1 {
      q = std::cmp::max(q, p[i - 1] + r[j - i]);
    }
    r[j] = q
  }
  return r[n];
}

#[test]
fn test_bottom_up_cut_rod() {
  let p = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
  assert_eq!(bottom_up_cut_rod(p, 10), 30);
}
