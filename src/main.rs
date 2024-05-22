use actix_web:: {
    HttpServer, App, web, HttpResponse, Responder
};

use actix_identity::Identity;


use actix_session::{ Session, SessionMiddleware, storage::SessionStore};



use serde::Serialize;

use tera::{ Tera, Context };




#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {

    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Bob")
        },
        Post {
            title: String::from("The Second Link"),
            link: String::from("https://example.com"),
            author: String::from("Alice")
        },
    ];

    data.insert("title", "teras");
    data.insert("name", "Rust");
    data.insert("posts", &posts);


    let _rendered = tera.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(_rendered)


}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(tera)
            .route("/", web::get().to(index))     
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}