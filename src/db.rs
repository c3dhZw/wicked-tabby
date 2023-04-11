use sqlite::{Connection, State};
use tiny_http::Request;

use crate::{server::{serve_error, serve_redirect}, base81::encode_base81};

struct url_awa {
    code: String,
    int: i64,
    redirect: String,
    expire_date: i64,
    can_expire: bool,
    userID: String,
    userIP: String,
    active: bool
}

impl url_awa {
    fn new(){

    }
}

fn gen_code(rederect: &String) -> String{
    let nv = rederect.parse::<u64>().unwrap();
    let lecode = encode_base81(nv);
    lecode
}

pub fn new_db_request(mut request: Request, database: &Connection, addQuery: &str) -> Result<(), anyhow::Error> {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    let parts = content.split("|").collect::<Vec<_>>();
    let mut redirect = parts[0].to_owned();
    
    if(&redirect[0..8] != "https://" || &redirect[0..7] != "http://"){
        let awa = "https://";
        redirect =  format!("{}{}", awa, redirect);
    }

    let code = gen_code(&redirect);
  
    let expire_date = parts[1].parse::<i64>().unwrap_or(0);
    let can_expire = parts[2].parse::<i64>().unwrap_or(0);
    let userID = parts[3];
    let userIP = "0.0.0.0";
    let awa = database
  .prepare(addQuery)
  .unwrap()
  .into_iter()
   .bind((1, code.as_str()))? //id
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
  }

  pub fn get_redirect(request: Request, database: &Connection, siteQuery: &str) -> Result<(), anyhow::Error> {
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
    dbg!("failed to get");
    serve_error(request, 404)
}