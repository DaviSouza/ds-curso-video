#![allow(proc_macro_derive_resolution_fallback)]
use crate::alunos::Aluno;
use crate::mongo_connection::DbConn;
use crate::relatorios::RelatorioFaturamento;

use bson::{Bson, Document};
use mongodb::sync::{Cursor, Collection};
use mongodb::{
    bson::doc,
    error::Error,
};


pub fn gerar_relatorio_faturamento(
    db_conn: &DbConn,
    nome_curso: &String,
) -> Result<Vec<RelatorioFaturamento>, Error> {
    let collection:Collection<Aluno> = db_conn.db.collection("aluno");
    let unwind = doc! {"$unwind": "$cursos"};
    let unwind_curso_info = doc! {"$unwind": "$curso_info"};
    let lookup = doc! {
        "$lookup": {
            "from": "curso", 
            "localField": "cursos",
            "foreignField": "_id",
            "as": "curso_info"
        }
    };
    let group = doc! {
        "$group": {
            "_id": "$cursos",
            "nome_curso": { "$first": "$curso_info.nome" },
            "total_alunos": { "$sum": 1 },
            "faturamento_curso": { "$sum": "$curso_info.preco" }
        }
    };
    let project = doc! {
                        "$project": {
                            "_id": 0,
                            "nome_curso": 1,
                            "total_alunos": 1,
                            "faturamento_curso": 1
                        }
                    };

    let pipeline = if nome_curso.is_empty() {
        vec![unwind, lookup, unwind_curso_info, group, project,]
    } else {
        vec![
            unwind,
            doc! {
                "$match": {
                    "nome_curso": nome_curso
                }
            },group,project,
        ]
    };

    let cursor: Cursor<Document> = collection.aggregate(pipeline, None)
    .ok()
    .expect("Error to generate report");
    let mut relatorio_vec: Vec<RelatorioFaturamento> = Vec::new();

    for result in cursor {
        if let Ok(doc) = result {
            let relatorio: RelatorioFaturamento = bson::from_bson(Bson::Document(doc))?;
            relatorio_vec.push(relatorio);
        }
    }

    Ok(relatorio_vec)
}
