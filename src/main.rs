use rocket::http::Method;
use rocket::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

static mut LED_STATUS: bool = false;

#[post("/on")]
fn on() -> &'static str {
    unsafe {
        LED_STATUS = true;
    }
    "true"
}

#[post("/off")]
fn off() -> &'static str {
    unsafe {
        LED_STATUS = false;
    }
    "false"
}

#[post("/toggle")]
fn toggle() -> String {
    unsafe {
        LED_STATUS = !LED_STATUS;
    }
    format!("{}", get_status().parse::<bool>().unwrap())
}

#[get("/status")]
fn get_status() -> String {
    unsafe { format!("{}", LED_STATUS) }
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Couldn't Build CORS")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(make_cors())
        .mount("/api/led/", routes![on, off, toggle])
}
