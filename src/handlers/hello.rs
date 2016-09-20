use iron::prelude::*;
use iron::status;
use router::Router;
use smzdh_commons::scredis;
use redis::Commands;
use redis;
use smzdh_commons::headers::{JsonResponse,success_json_response};
use smzdh_commons::errors::SError;

#[allow(dead_code)]
pub fn redis_handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let _ = scredis::redis_conn().map(|c| {
        let _:Result<(),redis::RedisError> = c.set("paomian", 42);
    });
    Ok(Response::with((status::Ok, query)))
}

#[allow(dead_code)]
pub fn postgres_handler(_: &mut Request) -> IronResult<Response> {
    pconn!(pc);
    let result = stry!(pc.query("SELECT * from users;", &[]));
    let mut response = JsonResponse::new();
    let mut vec = Vec::<i32>::new();
    for row in &result {
        vec.push(row.get("id"));
    }
    response.set_result(&vec);
    success_json_response(&response)
}

pub fn test(_: &mut Request) -> IronResult<Response> {
    let mut response = JsonResponse::new();
    response.set_result("pong");
    success_json_response(&response)
}

pub fn error_test(_:&mut Request) -> IronResult<Response> {
    let a:Result<i32,i32> = Err(0);
    let _ = stry!(a.map_err(|_| SError::None),"A 的值应该为一个数组。");
    Ok(Response::with((status::Ok, "hello")))
}
