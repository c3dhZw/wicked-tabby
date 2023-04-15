use json::object;
use sqlx::types::chrono::NaiveDateTime;
use tiny_http::Request;
use url::Url;

use crate::base81::encode_base81;
use crate::database::Database;
use crate::server::{serve_error, serve_redirect, serve_json, serve_json_error};
use crate::snowflakes::Snowflakes;

pub struct UrlDto {
  pub id: String,
  pub user_id: String,
  pub ip: String,
  pub url: String,
  pub clickies: i64,
  pub can_expire: bool,
  pub expire_time: Option<NaiveDateTime>,
  pub alive: bool,
}

pub async fn new_db_request(
  mut request: Request,
  database: &Database,
  snowflakes: &mut Snowflakes,
) -> anyhow::Result<()> {
  let mut content = String::new();
  request.as_reader().read_to_string(&mut content).unwrap();

  let parts = content.split("|").collect::<Vec<_>>();
  let mut redirect = parts[0].to_owned();

  if !redirect.starts_with("http://") && !redirect.starts_with("https://") {
    redirect = format!("{}{}", "https://", redirect);
  }

  let url = match Url::parse(&redirect) {
    Err(url::ParseError::RelativeUrlWithoutBase) => Url::parse(format!("https://{redirect}").as_str()),
    result => result,
  };

  let url = match url {
    Ok(url) => url,
    _ => {
      return serve_json_error(request, 420);
    }
  }
  .to_string();

  let code = encode_base81(snowflakes.generate() as u64);
  let a = code.clone();

  let expire_date = parts[1].parse::<i64>().unwrap_or(0);
  let can_expire = parts[2].parse::<i64>().unwrap_or(0);

  let user_id = parts[3];
  let user_ip = "0.0.0.0";

  let awa = sqlx::query!(
    "insert into urls (id, user_id, ip, url, expire_time, can_expire) values($1, $2, $3, $4, $5, $6)",
    code,
    user_id,
    user_ip,
    url,
    expire_date,
    can_expire,
  );

  let result = awa.execute(&database.pool).await;

  println!("awa {:?}", result);

  dbg!(code, url, expire_date, can_expire, user_id, user_ip);

  serve_json(request, a)
}

pub async fn get_db_request(
  mut request: Request,
  database: &Database,
  snowflakes: &mut Snowflakes,
) -> anyhow::Result<()> {
  let mut content = String::new();
  request.as_reader().read_to_string(&mut content).unwrap();

  let parts = content.split("|").collect::<Vec<_>>();
  let mut user_id = parts[0].to_owned();

  let awa = sqlx::query_as!(UrlDto, 
    "
    SELECT * 
    FROM urls 
    WHERE user_id = ($1)
    ", 
    user_id
  )
  .fetch_all(&database.pool)
  .await?;

  let mut data = json::JsonValue::new_array();

  for c in awa {
    let mut ljs = object!{
      "url": c.url,
      "id": c.id,
      "expire_time": c.expire_time.unwrap().timestamp()
    };

    data.push(ljs);
  }

  let nya = object!{
    "code": 200,
    "data": data
  };

  serve_json(request, json::stringify(nya))
}

pub async fn get_redirect(request: Request, database: &Database) -> anyhow::Result<()> {
  let url = request.url().clone();

  let segments: Vec<&str> = url.split('/').filter(|str| !str.is_empty()).collect();

  if segments.len() != 1 {
    return serve_error(request, 404);
  }

  let id = urlencoding::decode(segments[0])?;

  let result = sqlx::query_as!(UrlDto, "select * from urls where id = $1", id);

  match result.fetch_optional(&database.pool).await {
    Ok(Some(url)) => {
      let url = url.url;

      // redirect
      serve_redirect(request, url)?;

      println!("direbeute");

      return Ok(());
    }
    Err(err) => {
      println!("awa error!: {:?}", err.as_database_error());
    }
    _ => {}
  };

  serve_error(request, 404)
}
