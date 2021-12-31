#![allow(dead_code)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use sqldb::handler::*;
// use ipnetwork::IpNetwork;
// use std::collections::HashMap;
// use std::io::BufRead;
// use std::{
//     io::{BufReader, Write},
//     net::{TcpListener, TcpStream},
// };

mod enums;
pub mod graphql;
pub mod models;
mod sqldb;
mod utils;
use graphql::Query;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type SchemaType = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool = create_pool();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .extension(async_graphql::extensions::Logger)
        .finish();

    // let prefix_store = create_prefix_store(&connection);

    println!("Playground: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// fn handle_client(mut stream: TcpStream, prefix_store: &HashMap<i64, Vec<IpNetwork>>) {
//     let connection = establish_connection();
//     let mut buf = String::new();
//     let mut reader = BufReader::new(stream.try_clone().unwrap());

//     while reader.read_line(&mut buf).unwrap_or_default() > 0 {
//         println!("Received query {}", buf.trim());
//         if buf.trim() == "!!" {
//             buf = String::new();
//             continue;
//         }
//         if buf.trim().starts_with("!n") {
//             stream.write(b"C\n");
//             buf = String::new();
//             continue;
//         }
//         if buf.trim() == "!a" {
//             stream.write(b"F Missing required set name for A query\n");
//             buf = String::new();
//             continue;
//         }
//         if buf.trim() == "!q" {
//             return;
//         }
//         let members = members_for_as_set_recursive(&connection, &buf.trim()[3..]);
//         println!("{:?}", members);
//         let prefixes: Vec<IpNetwork> = prefixes_for_origins(&connection, &members, prefix_store);
//         let response = prefixes
//             .iter()
//             .map(|p| p.to_string())
//             .collect::<Vec<String>>()
//             .join(" ")
//             + "\n";
//         let mut wrapper = String::new();
//         write!(stream, "A{}\n{}C\n", response.len(), response);
//         buf = String::new();
//     }
// }

// fn socketserver() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:4343")?;
//     println!("building store");
//     let connection = establish_connection();
//     let prefix_store = create_prefix_store(&connection);
//     println!("done building store");

//     // accept connections and process them serially
//     for stream in listener.incoming() {
//         println!("New client: {:?}", stream);
//         handle_client(stream?, &prefix_store);
//     }
//     Ok(())
// }
