use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn home_page() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <html>
            <head>
                <title>IntelliStore API</title>
            </head>
            <body>
                <h1>IntelliStore API</h1>
                <p>Welcome to the IntelliStore API. This is the API for the IntelliStore project.</p>
            </body>
        </html>
        "#,
    )
}
