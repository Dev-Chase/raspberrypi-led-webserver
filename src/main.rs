use rocket::fs::FileServer;
use rocket::{fs::relative, fs::NamedFile, *};

static mut LED_STATUS: bool = false;

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("./static/index.html").await.ok()
}

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/api/led/", routes![on, off, toggle])
}
