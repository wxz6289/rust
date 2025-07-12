use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Deserialize)]
struct GCDParams {
    a: u64,
    b: u64,
}

async fn get_index() -> HttpResponse   {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>GCD Calculator</title>
            </head>
            <body>
                <h1>Welcome to the GCD Calculator</h1>
                <form action="/gcd" method="post">
                    <input type="number" name="a" placeholder="Enter first number" required>
                    <input type="number" name="b" placeholder="Enter second number" required>
                    <button type="submit">Calculate GCD</button>
                </form>
            </body>
            </html>
        "#)
}

async fn post_gcd(params: web::Form<GCDParams>) -> HttpResponse {
    if params.a == 0 || params.b == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Both numbers must be non-zero.");
    }

    let gcd = gcd(params.a, params.b);
    HttpResponse::Ok().body(format!("The GCD of {} and {} is {}", params.a, params.b, gcd))
}

fn gcd(x: u64, y: u64) -> u64 {
    let d = x % y;
    if d == 0 {
        y
    } else {
        gcd(y, d)
    }
}