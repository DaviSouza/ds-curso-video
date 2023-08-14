#![allow(proc_macro_derive_resolution_fallback)]
use crate::cursos::{Curso, InsertableCurso};
use crate::mongo_connection::{DbConn, Error::MongoSerializeBsonError};

use mongodb::sync::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    options::UpdateOptions,
    results::{InsertOneResult, UpdateResult, DeleteResult},
};

const COLLECTION: &str = "curso";

pub fn find(db_conn: &DbConn) -> Result<Vec<InsertableCurso>, Error> {
    let cursor = db_conn
        .db
        .collection(COLLECTION)
        .find(None, None)
        .ok()
        .expect("Erro ao carregar a lista de cursos");
    let items: Vec<InsertableCurso> = cursor
        .map(|doc| InsertableCurso::from_curso(doc.unwrap()))
        .collect();
    Ok(items)
}

pub fn find_one(db_conn: &DbConn, id: &String) -> Result<InsertableCurso, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let cursor = db_conn
        .db
        .collection(COLLECTION)
        .find_one(filter, None)
        .ok()
        .expect("Erro ao carregar o curso");
    let curso: InsertableCurso = InsertableCurso::from_curso(cursor.unwrap());
    Ok(curso)
}

pub fn insert(curso: Curso, db_conn: &DbConn) -> Result<InsertOneResult, Error> {
    let curso = db_conn
        .db
        .collection(COLLECTION)
        .insert_one(curso, None)
        .ok()
        .expect("Error ao criar curso");
    Ok(curso)
}

pub fn update(
    insert_able_curso: &InsertableCurso,
    db_conn: &DbConn,
) -> Result<UpdateResult, Error> {
    let curso = InsertableCurso::to_curso(insert_able_curso.clone());
    let obj_id = curso.id;
    let filter = doc! {"_id": obj_id};
    let document = bson::to_document(&curso)
        .map_err(MongoSerializeBsonError)
        .unwrap();

    let new_doc = doc! {"$set": document.clone()};
    let options = UpdateOptions::builder().upsert(true).build();
    let collection: Collection<Curso> = db_conn.db.collection(COLLECTION);
    let updated_doc = collection
        .update_one(filter, new_doc, options)
        .ok()
        .expect("Error ao atualizar o curso");
    Ok(updated_doc)
}

pub fn delete(id: &String, db_conn: &DbConn) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let collection: Collection<Curso> = db_conn.db.collection(COLLECTION);
    let curso = collection
        .delete_one(filter, None)
        .ok()
        .expect("Error ao deletar curso");
    Ok(curso)
}
