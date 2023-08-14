//#![feature(decl_macro, proc_macro_hygiene)] 
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate mongodb;

use dotenv::dotenv;
use rocket::{Request, Build};
use rocket::Rocket;
pub mod mongo_connection;
pub mod alunos;
pub mod cursos;
pub mod relatorios;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
/* 
#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db_conn = request.rocket().state::<mongo_connection::DbConn>().unwrap();
        Outcome::Success((db_conn))
    }
}*/
#[get("/")]
fn hello() -> &'static str {
    "Aplicação de vídeo aula em Rust e Mongodb"
}

#[launch]
pub fn rocket() -> Rocket<Build>  {
    dotenv().ok();
    let db_conn = mongo_connection::DbConn::new();
    rocket::build()
        .manage(db_conn)
        .register("/",catchers![internal_error, not_found])
        .mount("/", routes![hello])
        .mount(
            "/alunos",
            routes![
                alunos::handler::all,    
                alunos::handler::post,
                alunos::handler::get_aluno,
                alunos::handler::put,
                alunos::handler::delete_aluno
            ],
        )
        .mount(
            "/cursos",
            routes![
                cursos::handler::all,    
                cursos::handler::post,
                cursos::handler::put,
                cursos::handler::get_curso,            
                cursos::handler::delete_curso
            ],
        )        
        .mount("/relatorios",routes![
            relatorios::handler::build_report_all,
            relatorios::handler::build_report_by_name
            ],
        )
}