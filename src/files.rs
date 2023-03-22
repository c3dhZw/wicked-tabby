use std::fs;

#[cfg(debug_assertions)]
fn read_file(path: &str) -> Result<String, std::io::Error> { fs::read_to_string(path) }

macro_rules! embed_file {
  ($name:ident, $filename:expr) => {
    #[cfg(debug_assertions)]
    pub fn $name() -> String {
      match read_file(concat!("src/", $filename)) {
        Ok(contents) => contents,
        Err(err) => panic!("Failed to read file: {:?}", err),
      }
    }

    #[cfg(not(debug_assertions))]
    pub fn $name() -> String { include_str!($filename).to_owned() }
  };
}

embed_file!(get_index_html, "web/index.html");
embed_file!(get_index_css, "web/index.css");
embed_file!(get_index_js, "web/index.js");
embed_file!(get_error_html, "web/error.html");
embed_file!(get_redirect_html, "web/redirect.html");
