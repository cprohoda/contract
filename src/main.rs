mod account;
mod contract;
mod webapp;

extern crate time;
extern crate rocket;

#[get("/contract")]
fn contract() -> &'static str {
	"awwww yeeee"
}

fn main() {
    rocket::ignite().mount("/contract", routes![contract]).launch();
}
