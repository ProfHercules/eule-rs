use std::{
  collections::HashMap,
  ops::{Index, Mul, Sub},
};

use rayon::prelude::*;

pub fn is_prime(num: u64) -> bool {
  if num == 2 || num == 3 {
    return true;
  }

  let mod6 = num % 6;
  if num % 2 == 0 || (mod6 != 1 && mod6 != 5) {
    return false;
  }

  let mut i = 3;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }
    i += 2;
  }
  true
}

pub fn gen_primes(lim: u64) -> Vec<u64> {
  (2..=lim)
    .into_par_iter()
    .filter(|x| {
      let val = *x;

      (val == 2 || val % 2 == 1) && is_prime(val)
    })
    .collect()
}

pub fn is_palin<T: ToString>(s: T) -> bool {
  let s = s.to_string();

  let len = s.len();
  let mid = len >> 1;

  let first_half = s[..=mid].as_bytes();
  let second_half = s[mid..].as_bytes();

  for i in 0..mid {
    if first_half[i] != second_half[mid - (i + 1)] {
      return false;
    }
  }
  true
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
  let mut r: u64;

  while b != 0 {
    r = a % b;
    a = b;
    b = r;
  }
  a
}

pub fn lcm(a: u64, b: u64) -> u64 {
  let abs_ab = a * b;
  abs_ab / gcd(a, b)
}

pub fn factor_count_v2(n: u64, primes: &Vec<u64>) -> u64 {
  if n == 1 || n == 2 || n == 3 {
    return n;
  }
  let mut p_idx: usize = 0;
  let mut exp = 0;

  let mut result = 1;
  let mut num = n;

  while num > 1 {
    if num % primes[p_idx] == 0 {
      exp += 1;
      num = num / primes[p_idx];
    } else {
      if exp > 0 {
        result *= exp + 1;
      }
      exp = 0;
      p_idx += 1;
    }
  }
  if exp > 0 {
    result *= exp + 1;
  }

  result
}

// fn naive_factor_count(n: u64) -> u64 {
//   let lim = n >> 1;

//   let result: u64 = (2..=lim).into_par_iter().map(|i| if n % i == 0 { 1 } else { 0 }).sum();

//   result + 2
// }

fn large_num_to_vec(num: String, len: usize) -> Vec<u8> {
  format!("{:0>w$}", num, w = len)
    .as_bytes()
    .into_iter()
    .map(|d| d - 48)
    .rev()
    .collect()
}

pub fn add_large_nums(a: String, b: String) -> String {
  let max_len = a.len().max(b.len());

  let a = large_num_to_vec(a, max_len);
  let b = large_num_to_vec(b, max_len);

  let mut carry = 0;
  let mut result: Vec<u8> = Vec::new();

  for i in 0..max_len {
    let mut s = a[i] + b[i] + carry;
    carry = 0;
    if s > 9 {
      carry = 1;
      s = s - 10;
    }
    result.push(s);
  }
  if carry > 0 {
    result.push(carry);
  }

  result = result.into_iter().rev().map(|d| d + 48).collect();

  String::from_utf8_lossy(&result).to_string()
}

pub fn collatz_seq_len(start: u64) -> u64 {
  let mut chain_len = 1;
  let mut num = start;

  while num > 1 {
    if num % 2 == 0 {
      num = num >> 1;
    } else {
      num = 3 * num + 1;
    }
    chain_len += 1;
  }

  chain_len
}

pub fn binomial_coefficient(n: u64, k: u64) -> u64 {
  if k > n {
    return 0;
  }
  if k == 0 || k == n {
    return 1;
  }

  let k = k.min(n - k);

  let mut c = 1;
  for i in 0..k {
    c = c * (n - i) / (i + 1);
  }
  c
}

use super::data::*;

pub fn num_to_word(mut num: u64) -> String {
  let data = NUM_2_WORD_DATA;

  let mut map: HashMap<u64, String> = HashMap::new();

  for (num, name) in data.iter() {
    map.insert((*num) as u64, (*name).to_string());
  }

  if map.contains_key(&num) {
    return map[&num].clone();
  }

  let mut result: String = String::from("");

  if num >= 1000 {
    result += &format!("{}thousand", num_to_word(num / 1000)).to_string();
    num = num % 1000;
  }

  if num >= 100 {
    result += &format!("{}hundred", num_to_word(num / 100)).to_string();
    num = num % 100;
  }

  if result != "" && num > 0 {
    result += "and";
  }

  if map.contains_key(&num) {
    result += &format!("{}", map[&num].to_string());
  } else if num >= 90 {
    result += &format!("ninety{}", num_to_word(num % 90)).to_string();
  } else if num >= 80 {
    result += &format!("eighty{}", num_to_word(num % 80)).to_string();
  } else if num >= 70 {
    result += &format!("seventy{}", num_to_word(num % 70)).to_string();
  } else if num >= 60 {
    result += &format!("sixty{}", num_to_word(num % 60)).to_string();
  } else if num >= 50 {
    result += &format!("fifty{}", num_to_word(num % 50)).to_string();
  } else if num >= 40 {
    result += &format!("forty{}", num_to_word(num % 40)).to_string();
  } else if num >= 30 {
    result += &format!("thirty{}", num_to_word(num % 30)).to_string();
  } else if num >= 20 {
    result += &format!("twenty{}", num_to_word(num % 20)).to_string();
  } else if num >= 10 {
    result += &format!("{}teen", num_to_word(num % 10)).to_string();
  } else if num >= 1 {
    result += &format!("{}", num_to_word(num % 10)).to_string();
  }

  result.to_string()
}

/// returns the number of rows in a triangular array of given length
fn n(t: usize) -> usize {
  let t8 = 8.0 * t as f64;
  (0.5 * ((t8 + 1.0).sqrt() - 1.0)).round() as usize
}

pub fn find_path(mut arr: Vec<u64>) -> u64 {
  let last_row_len = n(arr.len());
  let mut row_len = last_row_len;

  let mut idx_start = arr.len() - last_row_len;
  let mut idx_end = idx_start + row_len;

  while idx_start > 0 {
    let mut prev_row_idx = idx_start - row_len + 1;

    for i in idx_start..idx_end - 1 {
      let max = arr[i].max(arr[i + 1]);
      let new_val = arr[prev_row_idx] + max;

      // let _ = std::mem::replace(&mut arr[prev_row_idx], new_val);
      arr[prev_row_idx] = new_val;

      prev_row_idx += 1;
    }

    row_len -= 1;
    idx_start -= row_len;
    idx_end = idx_start + row_len;
  }

  arr[0]
}

pub fn fac(n: u64) -> u64 {
  if n == 1 {
    return 1;
  }
  n * fac(n - 1)
}

pub fn dig_sum(mut num: u64) -> u64 {
  let mut sum = 0;

  while num > 0 {
    sum += num % 10;
    num /= 10;
  }
  sum
}
