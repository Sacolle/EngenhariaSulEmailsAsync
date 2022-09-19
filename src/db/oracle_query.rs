use oracle::Connection;
use chrono::NaiveDateTime;

use crate::db::chunks::OcorrenciaSoe;
use crate::error::TableProcessError;
use crate::INIVALS;

//conecta-se na base da empresa na oracle
//retorna hora_inicio, hora_fim, mensagem e agente da tabela
pub fn ocor_soe(empresa:&str, id:i32,)->Result<Vec<OcorrenciaSoe>,TableProcessError>{
	let conn = Connection::connect(
		&INIVALS.oracle_user,
		&INIVALS.oracle_senha,
		&INIVALS.oracle_url)?;

	let query = "SELECT hora_inicio, hora_fim, mensagem, agente FROM :1 WHERE ocor_id = :2";

	let rows = conn.query_as::<(
		NaiveDateTime,
		NaiveDateTime,
		Option<String>,
		Option<String>
	)>(query, &[&empresa,&id])?;

	let res = rows.into_iter()
		.filter_map(|row|row.ok()
			.map(|r|{
				//malabarismo para o borrow checker
				OcorrenciaSoe::new(r.0,r.1,r.2,r.3)
			})
		).collect::<Vec<OcorrenciaSoe>>();
	Ok(res)
}