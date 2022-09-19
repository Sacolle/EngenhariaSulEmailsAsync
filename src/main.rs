mod db;
mod io;
mod error;

#[macro_use]
extern crate lazy_static;

use chrono::Datelike;
use tokio::{fs as tfs, io::AsyncWriteExt};
use tera::Tera;
use sqlx::mysql::MySqlPoolOptions;
use tokio_stream::{self as stream, StreamExt};

use io::{templating::{tera_round_n_places,build_from_template},send_email};
use error::TableProcessError;
use db::{query,oracle_query};

lazy_static!(
	pub	static ref INIVALS: std::collections::HashMap<&'static str,String> = io::ini::load_config("config.ini");
	pub static ref TEMPLATE: Tera = {
		let mut t = Tera::new("templates/*").unwrap();
		t.register_function("round",  tera_round_n_places);
		t
	};
);

#[tokio::main]
async fn main() {
	let day = chrono::Utc::now();
	let mut err_file = tfs::OpenOptions::new()
		.append(true)
		.create(true)
		.write(true)
		.open(format!("{}-{}_err.txt",day.year(),day.month())).await
		.expect("Falha em gerar o arquivo de erro");

	let mut log_file = tfs::OpenOptions::new()
		.append(true)
		.create(true)
		.write(true)
		.open(format!("{}-{}_log.txt",day.year(),day.month())).await
		.expect("Falha em gerar o arquivo de log");
	

	if let Err(e) = laco_de_operacao(&mut log_file,&mut err_file).await{
		assert!(log_error(&mut err_file,"ROOT", e).await.is_ok());
	};
}

async fn laco_de_operacao(log_f:&mut tfs::File, err_f:&mut tfs::File)->Result<(),TableProcessError>{

	let email_table_connect = MySqlPoolOptions::new()
		.max_connections(5)
		.connect(
			&format!("{}{}",
				INIVALS.get("maria_url").unwrap(),
				INIVALS.get("maria_emaildb").unwrap())
			).await?;

	let empresas = query::empresas(&email_table_connect).await;

	let mut handles = Vec::new();
	for empresa in empresas{
		let emails = query::emails(&email_table_connect, &empresa).await;
		handles.push(tokio::spawn(async move {
			let res = process_table(&empresa,emails).await;
			(empresa, res)
		}));
	}
	let mut stream = stream::iter(handles);
	while let Some(result) = stream.next().await{
		let (emp,res) = result.await?;
		match res{
			Ok(_ids)=> {
				println!("Tabela {} acessada com sucesso",&emp);
				if let Some(ids) = _ids{
					log_emails(log_f, &emp, ids).await?;
				}
			},
			Err(e)=> {
				println!("Falha na tabela {}",&emp);
				log_error(err_f, &emp,e).await?;
			}
		}
	}
	Ok(())
}
async fn process_table(empresa:&str,destinos:Vec<(Option<String>,Option<String>)>)->Result<Option<Vec<i32>>,TableProcessError>{
	let mut sent_emails = Vec::new();

	let pool = MySqlPoolOptions::new()
		.max_connections(10)
		.connect( &format!("{}SGO_{}", INIVALS.get("maria_url").unwrap(), empresa)).await?;

	let ocorrencias = query::ocorrencias(&pool).await;
	if ocorrencias.is_empty(){
		println!("Nenhum resultado da tabela SGO_{}",empresa);
		return Ok(None);
	}

	for ocor in ocorrencias{
		let id = ocor.id();
		let (se,al,eqp) = ocor.eqp();

		let (mut soes,eqps) = tokio::join!(
			query::ocorrencias_soe(&pool, id),
			query::equipamentos(&pool, se, al, eqp, &id)
		);
		if soes.is_empty(){
			println!("não foi encontrado nenhu valor de soe na tabela da empresa {}, procurando na ORACLE",empresa);
			soes = oracle_query::ocor_soe(empresa, id)?;
		}

		let (title,body) = build_from_template(empresa, ocor, soes, eqps)?;
		send_email::send_email(&destinos, title, body).await?;
		query::update_ocorrencias(&pool, id).await;
		sent_emails.push(id);
	}
	Ok(Some(sent_emails))
}

async fn log_error(f:&mut tfs::File,empresa:&str,error:TableProcessError)-> Result<(),TableProcessError>{
	let now = chrono::Utc::now();	
	let error_msg = format!("{} at {}: {}\n",now,empresa,error);

	f.write_all(error_msg.as_bytes()).await?;
	Ok(())
}

async fn log_emails(f:&mut tfs::File,empresa: &str, ids: Vec<i32>)-> Result<(),TableProcessError>{
	let now = chrono::Utc::now();	
	let emails_sent = format!("{}: {} enviou emails com ids: {:?}\n",now, empresa, ids);

	f.write_all(emails_sent.as_bytes()).await?;
	Ok(())
}

#[cfg(test)]
mod tests{
	use super::*;
	use db::chunks::*;
	#[tokio::test]
	async fn build_template(){
		let empresa = "CERIM";
		let pool = MySqlPoolOptions::new()
			.max_connections(10)
			.connect( &format!("{}SGO_{}", INIVALS.get("maria_url").unwrap(),empresa)).await.unwrap();

		let ocor = sqlx::query_as::<_,Ocorrencia>(
			"
			SELECT OcoId, SE, AL, EQP, DtHrIni, DtHrFim, TipoOco, Faltas, CondPre, CondPos FROM Ocorrencia WHERE OcoId = 1; 
		")
		.fetch_one(&pool).await.unwrap();
		
		let id = 1;
		let (se,al,eqp) = ocor.eqp();

		let (mut soes,eqps) = tokio::join!(
			query::ocorrencias_soe(&pool, id),
			query::equipamentos(&pool, se, al, eqp, &id)
		);

		if soes.is_empty(){
			println!("não foi encontrado nenhu valor de soe na tabela da empresa {}, procurando na ORACLE",empresa);
			soes = oracle_query::ocor_soe(empresa, id).unwrap();
		}

		let (_,body) = build_from_template(empresa, ocor, soes, eqps).unwrap();
		let mut file = tfs::File::create("testres/testres.html").await.unwrap();
		assert!(file.write_all(body.as_bytes()).await.is_ok());
	}
}