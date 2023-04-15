use anyhow::bail;
use json::object;
use lazy_static::lazy_static;
use tiny_http::{Header, Method, Request, Response, Server};

use crate::database::Database;
use crate::db::{get_redirect, new_db_request, get_db_request};
use crate::errors::ERRORS;
use crate::files::{get_error_html, get_index_css, get_index_html, get_index_js, get_redirect_html};
use crate::snowflakes::Snowflakes;

pub struct ContentType;

impl ContentType {
  const CSS: &str = "text/css; charset=utf-8";
  const HTML: &str = "text/html; charset=utf-8";
  const JS: &str = "text/javascript; charset=utf-8";
  const JSON: &str = "text/json; charset=utf-8";
}

lazy_static! {
  static ref CSS_HEADER: Header = Header::from_bytes("Content-Type", ContentType::CSS).unwrap();
  static ref HTML_HEADER: Header = Header::from_bytes("Content-Type", ContentType::HTML).unwrap();
  static ref JS_HEADER: Header = Header::from_bytes("Content-Type", ContentType::JS).unwrap();
  static ref JSON_HEADER: Header = Header::from_bytes("Content-Type", ContentType::JSON).unwrap();
}

pub async fn start(address: &str) -> anyhow::Result<()> {
  let database = Database::new("tk.sqlite").await?;

  // id      | clickies | url                | expires      | user-id      |
  // ip-used         | alive
  // --------|----------|--------------------|--------------|--------------|-----------------|-------------
  // gyfhujs | 2        | https://google.com | 1678417266   | awa-owo-uwu  | 192.168.0.3     | true

  let server = match Server::http(address) {
    Ok(s) => s,
    Err(err) => {
      bail!("could not start HTTP server at {address}: {err}")
    }
  };

  // get the machine and node id from somewhere maybe
  let mut snowflakes = Snowflakes::new(0, 0);

  for request in server.incoming_requests() {
    match serve_request(request, &database, &mut snowflakes).await {
      Ok(_) => {}
      Err(err) => {
        log::error!("could not serve the response: {err}");
      }
    }
  }

  bail!("the server socket has shutdown")
}

async fn serve_request(request: Request, database: &Database, snowflakes: &mut Snowflakes) -> anyhow::Result<()> {
  log::info!(
    "received request! method: {:?}, url: {:?}",
    request.method(),
    request.url()
  );

  match (request.method(), request.url()) {
    (Method::Get, "/index.js") => serve_static(request, get_index_js(), JS_HEADER.clone()),
    (Method::Get, "/index.css") => serve_static(request, get_index_css(), CSS_HEADER.clone()),
    (Method::Get, "/") | (Method::Get, "/index.html") => serve_static(request, get_index_html(), HTML_HEADER.clone()),
    (Method::Post, "/create") => new_db_request(request, database, snowflakes).await,
    (Method::Post, "/get_existing") => get_db_request(request, database, snowflakes).await,
    (Method::Get, "/500") => serve_error(request, 500),
    //(Method::Get, "/favicon.ico") => serve_error(request, 404),
    _ => match get_redirect(request, database).await {
      Ok(value) => Ok(value),
      Err(value) => return Err(value),
    },
  }
}



pub fn serve_error(request: Request, code: usize) -> anyhow::Result<()> {
  let mut html = get_error_html();

  let message = match ERRORS.get(&code) {
    Some(message) => message,
    None => "",
  };

  html = html.replace("{error_message}", message);
  html = html.replace("{error_code}", &code.to_string());

  Ok(request.respond(Response::from_string(html).with_header(HTML_HEADER.clone()))?)
}

pub fn serve_json(request: Request, code: String) -> anyhow::Result<()> {
  Ok(request.respond(Response::from_string(code).with_header(JSON_HEADER.clone()))?)
}

pub fn serve_json_error(request: Request, code: usize) -> anyhow::Result<()> {
  let mut json = object!{
    "code": code,
  };

  Ok(request.respond(Response::from_string(json::stringify(json)).with_header(JSON_HEADER.clone()))?)
}

fn serve_static(request: Request, file_data: String, content_type: Header) -> anyhow::Result<()> {
  Ok(request.respond(Response::from_string(file_data).with_header(content_type))?)
}

pub fn serve_redirect(request: Request, redirect: String) -> anyhow::Result<()> {
  let mut html = get_redirect_html();

  html = html.replace("{url}", &redirect);

  Ok(request.respond(Response::from_string(html).with_header(HTML_HEADER.clone()))?)
}
