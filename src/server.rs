use anyhow::bail;
use lazy_static::lazy_static;
use sqlite::{Connection, State};
use tiny_http::{Header, Method, Request, Response, Server};

//use crate::db::{init_db};
use crate::errors::ERRORS;
use crate::files::{get_error_html, get_index_css, get_index_html, get_index_js, get_redirect_html};

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
  let database = sqlite::open("tk").unwrap();

  let startQuery = "
      CREATE TABLE if not exists urls (id TEXT, clickies INTEGER, url TEXT, expires INTEGER, canexpire INTEGER, userid TEXT, ip TEXT, alive INTEGER);
  ";
  database.execute(startQuery).unwrap();

  // id      | clickies | url                | expires      | user-id      | ip-used         | alive
  // --------|----------|--------------------|--------------|--------------|-----------------|-------------
  // gyfhujs | 2        | https://google.com | 1678417266   | awa-owo-uwu  | 192.168.0.3     | true

  let server = match Server::http(address) {
    Ok(s) => s,
    Err(err) => {
      bail!("could not start HTTP server at {address}: {err}")
    }
  };

  for request in server.incoming_requests() {
    match serve_request(request, &database) {
      Ok(_) => {}
      Err(err) => {
        log::error!("could not serve the response: {err}");
      }
    }
  }

  bail!("the server socket has shutdown")
}

fn serve_request(mut request: Request, database:&Connection) -> anyhow::Result<()> {
  log::info!(
    "received request! method: {:?}, url: {:?}",
    request.method(),
    request.url()
  );

  let siteQuery = "SELECT * FROM urls WHERE id = ?";
  let addQuery = "INSERT INTO urls VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
  // INSERT INTO urls VALUES ('gyfhujs', 0, "https://google.com", 1678417266, "awa-owo-uwu", "192.168.0.3", 1);

  match (request.method(), request.url()) {
    // (Method::Post, "/api/search") => serve_api_search(model, request),
    (Method::Get, "/index.js") => serve_static(request, get_index_js(), JS_HEADER.clone()),
    (Method::Get, "/index.css") => serve_static(request, get_index_css(), CSS_HEADER.clone()),
    (Method::Get, "/") | (Method::Get, "/index.html") => serve_static(request, get_index_html(), HTML_HEADER.clone()),
    (Method::Post, "/create") => {
      let mut content = String::new();
      request.as_reader().read_to_string(&mut content).unwrap();

      let parts = content.split("|").collect::<Vec<_>>();
      //poggers.com|2023-03-01 |true      |27aac518-64b0-4000-ade6-e29ae0c99800
      //url        |expire-date|can expire|user id

      let code = "womp";
      let mut redirect = parts[0].to_owned();

      if(&redirect[0..8] != "https://" || &redirect[0..7] != "http://"){
        let awa = "https://";
        redirect =  format!("{}{}", awa, redirect);

      }

      let expire_date = parts[1].parse::<i64>().unwrap_or(0);
      let can_expire = parts[2].parse::<i64>().unwrap_or(0);
      let userID = parts[3];
      let userIP = "0.0.0.0";

      let awa = database
      .prepare(addQuery)
      .unwrap()
      .into_iter()
       .bind((1, code))? //id
       .bind((2, 0))? //clicks
       .bind((3, redirect.as_str()))? //url redirect
       .bind((4, expire_date))? //when expires
       .bind((5, can_expire))? //user id
       .bind((6, userID))?
       .bind((7, userIP))?
       .bind((8, 1));
      awa.unwrap().next();

      dbg!(code, redirect, expire_date, can_expire, userID, userIP);
      serve_error(request, 420)
    },
    (Method::Get, "/500") => serve_error(request, 500),
    //(Method::Get, "/favicon.ico") => serve_error(request, 404),

    _ => {

      let end = request.url().clone();
      let mut end = end.chars();
      end.next();
      let end = end.as_str();

      let mut statement = database.prepare(siteQuery).unwrap();
      statement.bind((1, end)).unwrap();

      while let Ok(State::Row) = statement.next() {
          let redirect = statement.read::<String, _>("url").unwrap();
          if redirect != "" {
            return serve_redirect(request, redirect)
          }
      }
      
      dbg!("crap");
      serve_error(request, 404)
    },
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

fn serve_redirect(request: Request, redirect: String) -> anyhow::Result<()> {
  let mut html = get_redirect_html();

  html = html.replace("{url}", &redirect);

  Ok(request.respond(Response::from_string(html).with_header(HTML_HEADER.clone()))?)
}
