pub mod handler;
pub mod repository;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aluno {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nome: String,
    pub email: String,
    pub cursos: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableAluno {
    pub id:String,
    pub nome: String,
    pub email: String,
    pub cursos: Vec<ObjectId>,
}

impl InsertableAluno {
    fn from_aluno(aluno: Aluno) -> InsertableAluno {
        let id_aluno: String = aluno.id.unwrap().to_hex();
        InsertableAluno {
            id:id_aluno,
            nome: aluno.nome,
            email: aluno.email,
            cursos: aluno.cursos,
        }
    }
    pub fn to_aluno(aluno: InsertableAluno) -> Aluno {
        Aluno {
            id:Some(ObjectId::parse_str(aluno.id).unwrap()),
            nome: aluno.nome,
            email: aluno.email,
            cursos: aluno.cursos,
        }
    }
}