
use crate::mongo_connection::DbConn;
use crate::{alunos::Aluno, alunos::repository::{find, insert, find_one, update, delete}};
use mongodb::results::{InsertOneResult,UpdateResult};
use rocket::{http::Status, serde::json::Json, State};

use super::InsertableAluno;

/*
fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
} */

#[get("/")]
pub fn all(connection: &State<DbConn>) -> Result<Json<Vec<Aluno>>, Status> {
    
    let alunos_res = find(&connection);
    match alunos_res {
        Ok(alunos_res) => Ok(Json(alunos_res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<path>")]
pub fn get_aluno(connection: &State<DbConn>, path: String) -> Result<Json<InsertableAluno>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let aluno_detail = find_one(&connection, &id);
    match aluno_detail {
        Ok(aluno) => Ok(Json(aluno)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", format = "application/json", data = "<alunos>")]
pub fn post(alunos: Json<Aluno>, connection: &State<DbConn>) -> Result<Json<InsertOneResult>, Status> {
    let aluno_res = insert(alunos.into_inner(), &connection); 
    match aluno_res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/", data = "<aluno>")]
pub fn put(
    db: &State<DbConn>,
    aluno: Json<InsertableAluno>,
) -> Result<Json<UpdateResult>, Status> { 
    if aluno.id.is_empty() {
        return Err(Status::BadRequest);
    };
    let aluno_res = update(&aluno, &db); 
    match aluno_res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<path>")]
pub fn delete_aluno(connection: &State<DbConn>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = delete(&id, &connection);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Aluno deletado com sucesso!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}