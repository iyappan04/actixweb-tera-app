use actix_web:: {
    HttpServer, App, web, HttpResponse, Responder
};

use actix_identity::Identity;

extern crate diesel;
use actix_session::{ Session, SessionMiddleware, storage::SessionStore};



use serde::Serialize;

use tera::{ Tera, Context };

pub mod schema;
pub mod models;


use diesel::{ Connection, prelude::*};
use diesel::pg::PgConnection;

use dotenv::dotenv;

use models::{User, NewUser};


#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}


fn db_connect() -> PgConnection {

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url))

}


async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    data.insert("title", "Register");

    let rendered = tera.render("signup.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn register(data: web::Form<NewUser>) -> impl Responder {


    use schema::users;

    let mut connect = db_connect();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>( &mut connect)
        .expect("Error on register the data");


    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))

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
            // .app_data(tera)
            .data(tera)
            .route("/", web::get().to(index))    
            .route("/signup", web::get().to(signup)) 
            .route("/signup", web::post().to(register))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    
}