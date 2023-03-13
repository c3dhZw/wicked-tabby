use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
  pub static ref ERRORS: HashMap<usize, &'static str> = {
    let mut map = HashMap::new();

    map.insert(404, "page not found");

    map
  };
}
