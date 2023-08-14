use crate::mongo_connection::DbConn;
use crate::{relatorios::RelatorioFaturamento, relatorios::repository::gerar_relatorio_faturamento};
use rocket::{http::Status, serde::json::Json, State};

#[get("/")]
pub fn build_report_all(connection: &State<DbConn>) -> Result<Json<Vec<RelatorioFaturamento>>, Status> {
    let path = String::from("");
    let cursos_res = gerar_relatorio_faturamento(&connection, &path);
    match cursos_res {
        Ok(cursos_res) => Ok(Json(cursos_res)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/<path>")]
pub fn build_report_by_name(connection: &State<DbConn>, path: String) ->Result<Json<Vec<RelatorioFaturamento>>, Status> {
    let id = path;
    let relatorios_vec = gerar_relatorio_faturamento(&connection, &id);
    match relatorios_vec {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}