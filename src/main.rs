use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    name: String,
    age: String,
}

async fn hello() -> impl Responder {
    return web::Json(vec![User {
        name: "greg".to_string(),
        age: "12".to_string(),
    }]);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //     HttpServer::new(|| App::new().route("/countries", web::get().to(get_country_list)))
    HttpServer::new(|| App::new().route("/dude", web::get().to(hello)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
// use serde::{Deserialize, Serialize};
// use sqlx::postgres::PgPool;

// #[derive(Serialize)]
// struct Country {
//     country_code: String,
//     country_name: String,
// }

// async fn get_country_list() -> impl Responder {
//     let mut vec: Vec<Country> = Vec::new();

//     vec.push(Country {
//         country_code: "PH".to_string(),
//         country_name: "Philippines".to_string(),
//     });
//     vec.push(Country {
//         country_code: "MY".to_string(),
//         country_name: "Malaysia".to_string(),
//     });
//     vec.push(Country {
//         country_code: "ID".to_string(),
//         country_name: "Indonesia".to_string(),
//     });

//     return web::Json(vec);
// }

// #[actix_web::main]
// async fn server() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/countries", web::get().to(get_country_list)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/hello", web::get().to(|| async { "Hello World!" })))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

// #[actix_web::main]
// #[tokio::main]
// async fn get_req() -> Result<(), sqlx::Error> {
//     // Create a connection pool
//     let pool = PgPool::connect("postgres://postgres:password@localhost/postgres").await?;

//     let rows = sqlx::query!("select * from todos;")
//         .fetch_one(&pool)
//         .await?;
//     println!("rows: {:?}", rows);
//     // for row in rows {
//     //     println!(
//     //         "- [{}] {}: {}",
//     //         if row.done { "x" } else { " " },
//     //         row.id,
//     //         &row.description,
//     //     );
//     // }

//     Ok(())
// }

// async fn test_get_req() -> impl Responder {}
