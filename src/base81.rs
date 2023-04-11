const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%()*+,-;=?@^_{|}~";
const BASE: u64 = CHARSET.len() as u64;

pub struct Base81 {
  pub value: String,
}

impl From<u64> for Base81 {
  fn from(value: u64) -> Self {
    Self {
      value: encode_base81(value),
    }
  }
}

pub fn encode_base81(mut n: u64) -> String {
  if n == 0 {
    return "0".to_string();
  }

  let mut buffer = vec![];

  while n > 0 {
    let rem = n % BASE;

    n /= BASE;

    buffer.push(CHARSET[rem as usize]);
  }

  buffer.reverse();

  String::from_utf8(buffer).unwrap()
}

#[cfg(test)]
mod test {
  use crate::base81::encode_base81;

  #[test]
  fn test_encode_base81() {
    let test_cases = vec![
      (0, "A"),
      (1234567890, "tKj%21M"),
      (9876543210, "ct*Jvlr"),
      (18446744073709551615, "fPNJzJU66I~"),
    ];

    for (n, expected) in test_cases {
      let encoded = encode_base81(n);

      assert_eq!(encoded, expected);
    }
  }
}
