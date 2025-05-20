use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};

struct RateLimiter {
    clients: Mutex<HashMap<IpAddr, Vec<Instant>>>
}

async fn protected_resource(
    req: HttpRequest,
    rate_limiter: web::Data<RateLimiter>
) -> impl Responder {
    let ip = req.peer_addr().map(|addr| addr.ip()).unwrap_or([127, 0, 0, 1].into());
    
    let mut clients = rate_limiter.clients.lock().unwrap();
    let now = Instant::now();
    
    let requests = clients.entry(ip).or_insert_with(Vec::new);
    
    requests.retain(|&time| time > now - Duration::from_secs(60));
    
    if requests.len() < 5 {
        requests.push(now);
        HttpResponse::Ok().body("Access granted to protected resource!")
    } else {
        HttpResponse::TooManyRequests().body("Rate limit exceeded. Try again later.")
    }
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! (unprotected endpoint)")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rate_limiter = web::Data::new(RateLimiter {
        clients: Mutex::new(HashMap::new())
    });
    
    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(rate_limiter.clone())
            .route("/", web::get().to(hello))
            .route("/protected", web::get().to(protected_resource))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
