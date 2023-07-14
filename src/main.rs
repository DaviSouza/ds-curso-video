use ds_curso_video::rocket;
#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}