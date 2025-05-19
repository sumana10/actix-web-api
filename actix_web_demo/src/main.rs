use actix_web::{
    get, post, web, App, HttpResponse, Responder, HttpServer
};
use std::thread;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[get("/thread-info")]
async fn thread_info() -> impl Responder {
    let thread_id = thread::current().id();
    HttpResponse::Ok().body(format!("Running on thread: {:?}", thread_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num_workers = num_cpus::get(); 
    println!("server running at http://127.0.0.1:3000");
    
    HttpServer::new(|| {
        println!("Worker thread started: {:?}", thread::current().id());

        App::new()
            .service(hello)
            .service(echo)
            .service(thread_info) // Add the new endpoint
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(num_workers) // Explicitly set the number of workers
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
