#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")] //es como un decorador (? cuando entre a la ruta / va a ejecutarse esta funcion
fn index() -> Template {    //el segundo parametro de render es para que pase algun dato
    Template::render("index", context!{
        title: "Rocket"
    }) 
}

#[get("/")]
fn info() -> &'static str{
    "info"
}

#[post("/")]
fn info_create() -> &'static str {
    "creating info"
}

#[put("/")]
fn info_update() -> &'static str {
    "updating info"
}

#[delete("/")]
fn info_delete() -> &'static str {
    "deleting info"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])//aca en routes![] meto las rutas
    .mount("/info", routes![info, info_create, info_delete, info_update])//puedo ir uniendo, asi tengo estas que comparten el mismo /algo
    .attach(Template::fairing())
}