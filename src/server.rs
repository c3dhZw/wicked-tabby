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

lazy_static! {
  static ref CSS_HEADER: Header = Header::from_bytes("Content-Type", ContentType::CSS).unwrap();
  static ref HTML_HEADER: Header = Header::from_bytes("Content-Type", ContentType::HTML).unwrap();
  static ref JS_HEADER: Header = Header::from_bytes("Content-Type", ContentType::JS).unwrap();
}

pub fn start(address: &str) -> anyhow::Result<()> {
  let server = match Server::http(address) {
    Ok(s) => s,
    Err(err) => {
      bail!("could not start HTTP server at {address}: {err}")
    }
  };

  for request in server.incoming_requests() {
    match serve_request(request) {
      Ok(_) => {}
      Err(err) => {
        log::error!("could not serve the response: {err}");
      }
    }
  }

  bail!("the server socket has shutdown")
}

fn serve_request(request: Request) -> anyhow::Result<()> {
  log::info!(
    "received request! method: {:?}, url: {:?}",
    request.method(),
    request.url()
  );

  match (request.method(), request.url()) {
    // (Method::Post, "/api/search") => serve_api_search(model, request),
    (Method::Get, "/index.js") => serve_static(request, get_index_js(), JS_HEADER.clone()),
    (Method::Get, "/index.css") => serve_static(request, get_index_css(), CSS_HEADER.clone()),
    (Method::Get, "/") | (Method::Get, "/index.html") => serve_static(request, get_index_html(), HTML_HEADER.clone()),
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

  Ok(request.respond(Response::from_string(html).with_header(HTML_HEADER.clone()))?)
}

fn serve_static(request: Request, file_data: String, content_type: Header) -> anyhow::Result<()> {
  Ok(request.respond(Response::from_string(file_data).with_header(content_type))?)
}
