#[allow(dead_code)]
pub fn bubble_sort(arr: &mut Vec<i32>) {
  for i in 0..arr.len() {
    for j in 0..(arr.len() - i - 1) {
      if arr[j + 1] < arr[j] {
        arr.swap(j, j + 1);
      }
    }
  }
}

#[allow(dead_code)]
fn selection_sort(arr: &mut Vec<i32>) {
  for i in 0..arr.len() {
    let mut j_min = i;
    for j in i + 1..arr.len() {
      if arr[j] < arr[i] {
        j_min = j;
      }
    }

    if j_min != i {
      arr.swap(i, j_min);
    }
  }
}

#[allow(dead_code)]
fn inserti_sort(arr: &mut Vec<i32>) {
  for i in 0..arr.len() {
    for j in (0..i).rev() {
      if arr[j + 1] < arr[j] {
        arr.swap(j, j + 1)
      }
    }
  }
}

pub fn heapify(arr: &mut Vec<i32>, size: usize, i: usize) {
  let mut largest = i;
  let left = 2 * i + 1;
  let right = 2 * i + 2;
  if left < size && arr[left] > arr[i] {
    largest = left;
  }

  if right < size && arr[right] > arr[largest] {
    largest = right;
  }
  if largest != i {
    arr.swap(i, largest);
    heapify(arr, size, largest);
  }
}

fn sift_down(arr: &mut Vec<i32>) {
  let heap_size = arr.len();
  for i in (0..(arr.len() / 2)).rev() {
    heapify(arr, heap_size, i);
  }
}

#[allow(dead_code)]
pub fn heap_sort(arr: &mut Vec<i32>) {
  sift_down(arr);

  let mut heap_size = arr.len();

  for i in (1..arr.len()).rev() {
    arr.swap(0, i);
    heap_size -= 1;
    heapify(arr, heap_size, 0);
  }
}

fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];

  while !left.is_empty() && !right.is_empty() {
    if left[0] <= right[0] {
      result.push(left.remove(0));
    } else {
      result.push(right.remove(0));
    }
  }

  while !left.is_empty() {
    result.push(left.remove(0));
  }
  while !right.is_empty() {
    result.push(right.remove(0));
  }
  return result;
}

#[allow(dead_code)]
pub fn merge_sort(arr: &mut Vec<i32>) -> Vec<i32> {
  if arr.len() < 2 {
    return arr.clone();
  }

  let mut left: Vec<i32> = vec![];
  let mut right: Vec<i32> = vec![];

  for i in 0..arr.len() {
    if i < arr.len() / 2 {
      left.push(arr[i])
    } else {
      right.push(arr[i])
    }
  }

  left = merge_sort(&mut left).to_vec();
  right = merge_sort(&mut right).to_vec();

  return merge(&mut left, &mut right);
}

fn partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
  let pivot = arr[(hi + lo) / 2];
  let mut i = lo;
  let mut j = hi;
  loop {
    while arr[i] < pivot {
      i += 1;
    }

    while arr[j] > pivot {
      j -= 1;
    }

    if i >= j {
      return j;
    }

    arr.swap(i, j);
  }
}

#[allow(dead_code)]
pub fn quicksort(arr: &mut Vec<i32>, lo: usize, hi: usize) {
  if lo < hi {
    let p = partition(arr, lo, hi);
    quicksort(arr, lo, p);
    quicksort(arr, p + 1, hi);
  }
}

// # Sort an array a[0...n-1].
// gaps = [701, 301, 132, 57, 23, 10, 4, 1]

// # Start with the largest gap and work down to a gap of 1
// foreach (gap in gaps)
// {
//     # Do a gapped insertion sort for this gap size.
//     # The first gap elements a[0..gap-1] are already in gapped order
//     # keep adding one more element until the entire array is gap sorted
//     for (i = gap; i < n; i += 1)
//     {
//         # add a[i] to the elements that have been gap sorted
//         # save a[i] in temp and make a hole at position i
//         temp = a[i]
//         # shift earlier gap-sorted elements up until the correct location for a[i] is found
//         for (j = i; j >= gap and a[j - gap] > temp; j -= gap)
//         {
//             a[j] = a[j - gap]
//         }
//         # put temp (the original a[i]) in its correct location
//         a[j] = temp
//     }
// }

// vec![1, 3, 2, 5, 4, 65, 32]
#[allow(dead_code)]
pub fn shell_sort(arr: &mut Vec<i32>) {
  // Marcin Ciura's gap sequence
  let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];

  for gap in gaps {
    for i in gap..arr.len() {
      let temp = arr[i];
      let mut j = i;
      while j >= gap && arr[j - gap] > temp {
        arr.swap(j, j - gap);
        j -= gap;
      }
      arr[j] = temp;
    }
  }
}

#[allow(dead_code)]
pub fn counting_sort(a: &mut Vec<i32>, k: i32) -> Vec<i32> {
  let mut count_array = vec![0; (k + 1) as usize];
  let mut result = vec![0; a.len()];

  for x in a.clone() {
    count_array[x as usize] += 1;
  }

  for i in 1..count_array.len() {
    count_array[i] = count_array[i - 1] + count_array[i];
  }

  for j in (0..result.len()).rev() {
    let n = a[j];
    let count = count_array[n as usize];
    result[count - 1] = n;
    count_array[n as usize] -= 1;
  }

  result
}

#[test]
fn bubble1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4];
  bubble_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

#[test]
fn selection1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4];
  selection_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

#[test]
fn insert1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4, 65, 32];
  inserti_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5, 32, 65]);
}

#[test]
fn heap1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4, 65, 32];
  heap_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5, 32, 65]);
}

#[test]
fn merge_sort1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4, 65, 32];
  heap_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5, 32, 65]);
}

#[test]
fn quicksort1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4, 65, 32];
  let hi = arr.len() - 1;
  quicksort(&mut arr, 0, hi);
  assert_eq!(arr, vec![1, 2, 3, 4, 5, 32, 65]);
}

#[test]
fn shell_sort1() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 5, 4, 65, 32];
  shell_sort(&mut arr);
  assert_eq!(arr, vec![1, 2, 3, 4, 5, 32, 65]);
}

#[test]
fn test_counting_sort() {
  let mut arr: Vec<i32> = vec![1, 3, 2, 4, 4, 2, 1];
  assert_eq!(counting_sort(&mut arr, 4), [1, 1, 2, 2, 3, 4, 4]);
}
