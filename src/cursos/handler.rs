use crate::cursos::InsertableCurso;
use crate::mongo_connection::DbConn;
use crate::{cursos::Curso, cursos::repository::{find,insert,update,find_one,delete}};
use mongodb::results::{InsertOneResult,UpdateResult};
use rocket::{http::Status, serde::json::Json, State};

/*
fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
} */

#[get("/")]
pub fn all(connection: &State<DbConn>) -> Result<Json<Vec<InsertableCurso>>, Status> {
    
    let cursos_res = find(&connection);
    match cursos_res {
        Ok(cursos_res) => Ok(Json(cursos_res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<path>")]
pub fn get_curso(connection: &State<DbConn>, path: String) -> Result<Json<InsertableCurso>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let curso_detail = find_one(&connection, &id);
    match curso_detail {
        Ok(curso) => Ok(Json(curso)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", format = "application/json", data = "<curso>")]
pub fn post(curso: Json<Curso>, connection: &State<DbConn>) -> Result<Json<InsertOneResult>, Status> {
    let curso_res = insert(curso.into_inner(), &connection); 
    match curso_res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/", data = "<curso>")]
pub fn put(
    db: &State<DbConn>,
    curso: Json<InsertableCurso>,
) -> Result<Json<UpdateResult>, Status> { 
    if curso.id.is_empty() {
        return Err(Status::BadRequest);
    };
    let curso_res = update(&curso, &db); 
    match curso_res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<path>")]
pub fn delete_curso(connection: &State<DbConn>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = delete(&id, &connection);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Curso deletado com sucesso!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}