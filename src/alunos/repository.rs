#![allow(proc_macro_derive_resolution_fallback)]
use crate::alunos::{Aluno, InsertableAluno};
use crate::mongo_connection::{DbConn, Error::MongoSerializeBsonError};

use mongodb::sync::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    options::UpdateOptions,
    results::{InsertOneResult, UpdateResult, DeleteResult},
};

const COLLECTION: &str = "aluno";

pub fn find(db_conn: &DbConn) -> Result<Vec<Aluno>, Error> {
    let cursor = db_conn
        .db
        .collection(COLLECTION)
        .find(None, None)
        .ok()
        .expect("Erro ao carregar a lista de alunos");
    let items = cursor.map(|doc| doc.unwrap()).collect();
    Ok(items)
}

pub fn find_one(db_conn: &DbConn, id: &String) -> Result<InsertableAluno, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let cursor = db_conn
        .db
        .collection(COLLECTION)
        .find_one(filter, None)
        .ok()
        .expect("Erro ao carregar o aluno");
    let aluno: InsertableAluno = InsertableAluno::from_aluno(cursor.unwrap());
    Ok(aluno)
}

pub fn insert(aluno: Aluno, db_conn: &DbConn) -> Result<InsertOneResult, Error> {
    let insertable = InsertableAluno::from_aluno(aluno.clone());
    let user = db_conn
        .db
        .collection(COLLECTION)
        .insert_one(insertable, None)
        .ok()
        .expect("Error ao criar aluno");

    Ok(user)
}

pub fn update(
    insert_able_: &InsertableAluno,
    db_conn: &DbConn,
) -> Result<UpdateResult, Error> {
    let aluno = InsertableAluno::to_aluno(insert_able_.clone());
    let obj_id = aluno.id;
    let filter = doc! {"_id": obj_id};
    let document = bson::to_document(&aluno)
        .map_err(MongoSerializeBsonError)
        .unwrap();

    let new_doc = doc! {"$set": document.clone()};
    let options = UpdateOptions::builder().upsert(true).build();
    let collection: Collection<Aluno> = db_conn.db.collection(COLLECTION);
    let updated_doc = collection
        .update_one(filter, new_doc, options)
        .ok()
        .expect("Error ao atualizar aluno");
    Ok(updated_doc)
}

pub fn delete(id: &String, db_conn: &DbConn) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let collection: Collection<Aluno> = db_conn.db.collection(COLLECTION);
    let aluno = collection
        .delete_one(filter, None)
        .ok()
        .expect("Error ao criar aluno");
    Ok(aluno)
}