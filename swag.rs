#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use simple_logger::SimpleLogger;
pub mod base81 {
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
    fn encode_base81(mut n: u64) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut buffer = ::alloc::vec::Vec::new();
        while n > 0 {
            let rem = n % BASE;
            n /= BASE;
            buffer.push(CHARSET[rem as usize]);
        }
        buffer.reverse();
        String::from_utf8(buffer).unwrap()
    }
}
pub mod server {
    use std::fs::File;
    use std::io;
    use anyhow::bail;
    use lazy_static::lazy_static;
    use tiny_http::{Header, Method, Request, Response, Server};
    use crate::errors::ERRORS;
    use crate::files::{get_error_html, get_index_css, get_index_html, get_index_js};
    pub struct ContentType;
    impl ContentType {
        const CSS: &str = "text/css; charset=utf-8";
        const HTML: &str = "text/html; charset=utf-8";
        const JS: &str = "text/javascript; charset=utf-8";
    }
    pub fn start(address: &str) -> anyhow::Result<()> {
        let server = match Server::http(address) {
            Ok(s) => s,
            Err(err) => {}
        };
        ();
        for request in server.incoming_requests() {
            match serve_request(request) {
                Ok(_) => {}
                Err(err) => {
                    ();
                }
            }
        }
        ()
    }
    fn serve_request(request: Request) -> anyhow::Result<()> {
        ();
        match (request.method(), request.url()) {
            (Method::Get, "/index.js") => {
                serve_static(request, get_index_js(), JS_HEADER.clone())
            }
            (Method::Get, "/index.css") => {
                serve_static(request, get_index_css(), CSS_HEADER.clone())
            }
            (Method::Get, "/") | (Method::Get, "/index.html") => {
                serve_static(request, get_index_html(), HTML_HEADER.clone())
            }
            (Method::Get, "/500") => serve_error(request, 500),
            _ => serve_error(request, 404),
        }
    }
    fn serve_error(request: Request, code: usize) -> anyhow::Result<()> {
        let mut html = get_error_html();
        let message = match ERRORS.get(&code) {
            Some(message) => message,
            None => "",
        };
        html = html.replace("{error_message}", message);
        html = html.replace("{error_code}", &code.to_string());
        Ok(
            request
                .respond(Response::from_string(html).with_header(HTML_HEADER.clone()))?,
        )
    }
    fn serve_static(
        request: Request,
        file_data: String,
        content_type: Header,
    ) -> anyhow::Result<()> {
        Ok(request.respond(Response::from_string(file_data).with_header(content_type))?)
    }
}
pub mod errors {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
}
pub mod files {
    use std::fs;
    #[cfg(debug_assertions)]
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }
    #[cfg(debug_assertions)]
    pub fn get_index_html() -> String {
        match read_file("src/web/index.html") {
            Ok(contents) => contents,
            Err(err) => {
                ::core::panicking::panic_fmt(
                    format_args!("Failed to read file: {0:?}", err),
                )
            }
        }
    }
    #[cfg(debug_assertions)]
    pub fn get_index_css() -> String {
        match read_file("src/web/index.css") {
            Ok(contents) => contents,
            Err(err) => {
                ::core::panicking::panic_fmt(
                    format_args!("Failed to read file: {0:?}", err),
                )
            }
        }
    }
    #[cfg(debug_assertions)]
    pub fn get_index_js() -> String {
        match read_file("src/web/index.js") {
            Ok(contents) => contents,
            Err(err) => {
                ::core::panicking::panic_fmt(
                    format_args!("Failed to read file: {0:?}", err),
                )
            }
        }
    }
    #[cfg(debug_assertions)]
    pub fn get_error_html() -> String {
        match read_file("src/web/error.html") {
            Ok(contents) => contents,
            Err(err) => {
                ::core::panicking::panic_fmt(
                    format_args!("Failed to read file: {0:?}", err),
                )
            }
        }
    }
}
fn main() -> anyhow::Result<()> {
    SimpleLogger::new().env().init()?;
    ();
    server::start("0.0.0.0:3069")?;
    Ok(())
}
