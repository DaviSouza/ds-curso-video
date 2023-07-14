pub mod handler;
pub mod repository;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub id_video: i32,
    pub descricao: String,
    pub titulo: String,
    pub link: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Curso {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nome: String,
    pub descricao: String,
    pub duracao: i32,
    pub preco: f32,
    pub classificacao:i32,
    pub videos: Vec<Video>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableCurso {
    pub id:String,
    pub nome: String,
    pub descricao: String,
    pub duracao: i32,
    pub preco: f32,
    pub classificacao:i32,
    pub videos: Vec<Video>
}

impl InsertableCurso {
    
    fn from_curso(curso: Curso) -> InsertableCurso {
        let id_curso: String = curso.id.unwrap().to_hex();  
        InsertableCurso {
            id:id_curso,
            nome: curso.nome,
            descricao: curso.descricao,
            duracao: curso.duracao,
            preco: curso.preco,
            classificacao: curso.classificacao,
            videos: curso.videos,
        }
    }
    fn to_curso(cursos: InsertableCurso) -> Curso {
        Curso {
            id:Some(ObjectId::parse_str(cursos.id).unwrap()),
            nome: cursos.nome,
            descricao: cursos.descricao,
            duracao: cursos.duracao,
            preco: cursos.preco,
            classificacao: cursos.classificacao,
            videos: cursos.videos,
        }
    }

}