pub mod handler;
pub mod repository;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatorioFaturamento {
    pub nome_curso: String,
    pub total_alunos: usize,
    pub faturamento_curso: f32,
}