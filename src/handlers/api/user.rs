use iron::prelude::*;
use router::Router;
use rand::{OsRng,Rng};
use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::errors::SmzdhError;
use smzdh_commons::middleware::Json;
use smzdh_commons::databases;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let _ = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let postgres_c = pconn!(req);
    let mut inner = headers::JsonResponse::new();
    inner.insert("username","paomian");
    inner.insert("password","hello");
    let test = headers::JsonResponse::new_with(0,"",&inner);
    headers::success_json_response(&test)
}

pub fn signup(req:&mut Request) -> IronResult<Response> {
    let json = sexpect!(req.extensions.get::<Json>(),
                        SmzdhError::ParamsError.to_response(
                            Some("body 必须是Json.".to_string())
                        )).clone();
    let object = sexpect!(json.as_object(),
                          SmzdhError::ParamsError.to_response(None));
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let postgres_c = pconn!(req);
    let result = stry!(databases::create_user(postgres_c,username,password));
    let mut inner = headers::JsonResponse::new();
    let test = headers::JsonResponse::new_with(0,"",&inner);
    headers::success_json_response(&test)
}

pub fn ec(_: &mut Request) -> IronResult<Response> {
    let me = "paomian";
    let mut rng = OsRng::new().ok().unwrap();
    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    let e = utils::encrypt(me.as_bytes(),&key,&iv).ok().unwrap();
    info!("e:{:?},key:{:?},iv:{:?}",utils::hex(&e),utils::hex(&key),utils::hex(&iv));
    info!("hello");
    headers::success_json_response(&headers::JsonResponse::new())
}
