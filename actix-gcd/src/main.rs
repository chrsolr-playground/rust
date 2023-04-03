use actix_web::{ web, App, HttpServer, HttpResponse };

struct GcdParameters {
    n: u64,
    m: u64,
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_index))
    }).bind(("127.0.0.1", 3000))
        .expect("ERROR 1")
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
              <input type="text" name="n">
              <input type="text" name="n">
              <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

async fn post_index(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("COCOCOCOCOOD");
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body("damdso")
}
