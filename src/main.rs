// use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::Method;
use rocket::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use rppal::gpio::{ Gpio, OutputPin };
use std::sync::Mutex;

static mut LED_STATUS: bool = false;

// #[get("/")]
// async fn home() -> Option<NamedFile> {
//     NamedFile::open("./static/index.html").await.ok()
// }

#[post("/on")]
fn on(led_pin: &State<Mutex<OutputPin>>) -> &'static str {
    led_pin.lock().unwrap().set_high();
    unsafe { LED_STATUS = true; }
    "true"
}

#[post("/off")]
fn off(led_pin: &State<Mutex<OutputPin>>) -> &'static str {
    led_pin.lock().unwrap().set_low();
    unsafe { LED_STATUS = false; }
    "false"
}

#[post("/toggle")]
fn toggle(led_pin: &State<Mutex<OutputPin>>) -> String {
    led_pin.lock().unwrap().toggle();
    unsafe { LED_STATUS = !LED_STATUS; }
    get_status()
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
    let gpio: Gpio = Gpio::new().unwrap();
    let led_pin: Mutex<OutputPin> = Mutex::new(gpio.get(18).unwrap().into_output());
    rocket::build()
        .manage(led_pin)
        .attach(make_cors())
        // .mount("/static", FileServer::from(relative!("static")))
        // .mount("/", routes![home])
        .mount("/led", routes![on, off, toggle, get_status])
}
