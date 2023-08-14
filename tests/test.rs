use bson::Document;
use ds_curso_video::alunos::Aluno;
use mongodb::{
    options::{InsertOneOptions, FindOptions, DeleteOptions}, results::{InsertOneResult, DeleteResult}};
use rocket::{route::BoxFuture, futures::io::Cursor};

use mockall::automock;
use mongodb::error::Result;

#[automock]
pub trait MockCollection{
    fn find(&self, filter: Document, options: Option<FindOptions>) -> BoxFuture<'static, Result<Cursor<Vec<Aluno>>>>;
    fn insert_one(&self, document: Aluno, options: Option<InsertOneOptions>) -> BoxFuture<'static, Result<InsertOneResult>>;
    fn delete_one(&self, filter: Document, options: Option<DeleteOptions>) -> BoxFuture<'static, Result<DeleteResult>>;
}

#[cfg(test)]
mod tests {
    use ds_curso_video::alunos::Aluno;
    use mongodb::bson::{doc, oid::ObjectId};
    use rocket::futures::io::Cursor;
    use crate::{MockMockCollection, MockCollection};

    #[tokio::test]
    async fn test_get_data_from_mongodb() {
        // Configurar o servidor de teste falso do MongoDB usando mockito
        let mut server = mockito::Server::new();

        let mut mock_collection = MockMockCollection::new();

        mock_collection.expect_find().returning(|_, _| {
            let obj1 = ObjectId::new();
            let obj2 = ObjectId::new();
            let obj3 = ObjectId::new();
            let obj4 = ObjectId::new();
            let alunos = vec![
                Aluno {
                    id: Some(ObjectId::new()),
                    nome: "Aluno 1".to_string(),
                    email: "aluno1@example.com".to_string(),
                    cursos: vec![obj1, obj2],
                },
                Aluno {
                    id: Some(ObjectId::new()),
                    nome: "Aluno 2".to_string(),
                    email: "aluno2@example.com".to_string(),
                    cursos: vec![obj3, obj4],
                },
            ];
            let cursor = Cursor::new(alunos);
            Box::pin(async move { Ok(cursor) })
        });

        let mock_collection= mock_collection;
        let doc_filter = doc! {};
        // Agora você pode executar suas operações de leitura no banco de dados usando a coleção mockada
        let cursor = mock_collection.find(doc_filter, None).await.ok()
        .expect("Erro ao carregar a lista de alunos");
        let alunos = cursor.into_inner();
        let mock_data = serde_json::to_string(&alunos);

        // Configurar o mockito para responder com os dados mockeados quando a API do MongoDB for chamada
        let mock = server.mock("GET", "/alunos")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_data.unwrap())
            .create();

        assert_eq!(&alunos[0].nome, "Aluno 1");
        mock.assert();
    }
}
