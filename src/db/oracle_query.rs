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
				OcorrenciaSoe::new(r.0,r.1,r.2,r.3)
			})
		).collect::<Vec<OcorrenciaSoe>>();
	Ok(res)
}

#[cfg(test)]
mod tests{
use super::*;
	#[test]
	fn conn_oracle(){
		let conn = Connection::connect(
			&INIVALS.oracle_user,
			&INIVALS.oracle_senha,
			format!("{}:{}/CERIM",&INIVALS.oracle_url,&INIVALS.oracle_port));
		if let Err(e) = conn{
			println!("{}",e);
			panic!();
		}
	}
	#[test]
	fn query_oracle(){
		let empresa = "CERIM";
		let id = 1;
		let conn = Connection::connect(
			&INIVALS.oracle_user,
			&INIVALS.oracle_senha,
			&INIVALS.oracle_url).unwrap();

		let query = "SELECT hora_inicio, hora_fim, mensagem, agente FROM :1 WHERE ocor_id = :2";

		let rows = conn.query_as::<(
			NaiveDateTime,
			NaiveDateTime,
			Option<String>,
			Option<String>
		)>(query, &[&empresa,&id]);

		assert!(rows.is_ok());
	}


}