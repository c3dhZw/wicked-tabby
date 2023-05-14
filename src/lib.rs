pub mod base81;
pub mod database;
pub mod db;
pub mod domain;
pub mod snowflakes;

use worker::{event, Context, Env, Request, Response, Result, RouteContext, Router};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
  let router = Router::new();

  router.post_async("/", create_url).run(req, env).await;

  Response::ok("Hello, World!")
}

async fn create_url(req: Request, ctx: RouteContext<()>) -> Result<Response> {
  // todo: yeah
  
  Response::ok("Hello, World!")
}
