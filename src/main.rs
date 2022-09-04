use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Book Store",
        "books":[
            {
                "name":"Harry Potter",
                "author":"J K Rowlings",
                "image_path":"/static/image/download.jpeg"
            },
            {
                "name":"Lord of the ring",
                "author":"Tolken",
                "image_path": "/static/image/lord_of.jpeg"
            },
            {
                "name":"Americanah",
                "author":"Chimamada Adichie",
                "image_path":"/static/image/americanah.jpeg"
            },
            {
                "name":"Elon Musk",
                "author":"#####",
                "image_path":"/static/image/elon.jpeg"
            },
        ]


    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)

    // Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
