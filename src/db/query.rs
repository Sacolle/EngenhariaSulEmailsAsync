use sqlx::mysql::MySqlPool;
use tokio_stream::StreamExt;

use crate::db::chunks::*;

///realiza o query das falhas anteriores deste equipamento, omitindo a instância atual do equipamento
pub async fn equipamentos(pool: &MySqlPool, se:&str, al:&str, eqp:&str, id:&i32)->Vec<PrevEqp>{
	let stream = sqlx::query_as::<_,PrevEqp>(
		"
		SELECT Faltas, DtHrOco, ProtSen, ProtAtu
		FROM Ocorrencia
		WHERE SE = ? and AL = ? and EQP = ? and not OcoId = ?
		ORDER BY DtHrOco DESC
		LIMIT 5;
		")
		.bind(se)
		.bind(al)
		.bind(eqp)
		.bind(id)
		.fetch(pool);
	
	stream.filter_map(|val|val.ok()).collect::<Vec<PrevEqp>>().await
}

//valores intemediários ao query
#[derive(sqlx::FromRow)]
struct Empresas(Option<String>);

pub async fn empresas(pool: &MySqlPool)->Vec<String>{
	let stream = sqlx::query_as::<_,Empresas>(
		"
		SELECT DISTINCT Empresa FROM CadastroEmails;
		")
		.fetch(pool);
	
	stream.filter_map(|v|v.ok().map(|emp|emp.0).flatten()).collect().await
}

//valores intemediários ao query
#[derive(sqlx::FromRow)]
struct Email(Option<String>,Option<String>);

pub async fn emails(pool: &MySqlPool,empresa: &str)->Vec<(Option<String>,Option<String>)>{
	let stream = sqlx::query_as::<_,Email>(
		"
		SELECT EmailName, EmailAddr FROM CadastroEmails WHERE Empresa = ?;
	")
	.bind(empresa)
	.fetch(pool);

	stream.filter_map(|value|value.ok()
		.map(|Email(a,b)|(a,b))
	).collect().await
}


pub async fn ocorrencias(pool:&MySqlPool)->Vec<Ocorrencia>{
	let stream = sqlx::query_as::<_,Ocorrencia>(
		"
		SELECT OcoId, SE, AL, EQP, DtHrIni, DtHrFim, TipoOco, Faltas, CondPre, CondPos FROM Ocorrencia LIMIT 10; 
	")
	.fetch(pool);

	stream.filter_map(|val|val.ok()).collect().await
}


pub async fn ocorrencias_soe(pool:&MySqlPool,id:i32)->Vec<OcorrenciaSoe>{
	let stream = sqlx::query_as::<_,OcorrenciaSoe>(
		"
		SELECT EventTime, E3TimeStamp, Mensagem, ActorID FROM Ocorrencia_SOE WHERE OcoId = ?;	
	")
	.bind(id)
	.fetch(pool);

	stream.filter_map(|val|val.ok()).collect().await
}

pub async fn update_ocorrencias(pool:&MySqlPool,id:i32){
	let rows_affected = sqlx::query(
		r#"
		UPDATE Ocorrencia_SOE SET EmailSended = "S" WHERE id = ?;
	"#)
	.bind(id)
	.execute(pool)
	.await
	.unwrap()	//FIXME: fazer error handling
	.rows_affected();

	assert!(rows_affected > 0);
}