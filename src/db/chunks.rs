use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};
use sqlx::types::Json;

///vai para o email
#[derive(sqlx::FromRow,Serialize)]
pub struct Ocorrencia{
	#[sqlx(rename = "OcoId")]
	id:  	 i32,
	#[sqlx(rename = "SE")]
	se:  	 String,
	#[sqlx(rename = "AL")]
	al:   	 String,
	#[sqlx(rename = "EQP")]
	eqp:  	 String,
	#[sqlx(rename = "DtHrIni")]
	#[serde(serialize_with = "date_to_str")]
	inicio:  NaiveDateTime,
	#[sqlx(rename = "DtHrFim")]
	#[serde(serialize_with = "date_to_str")]
	fim:     NaiveDateTime,
	#[sqlx(rename = "TipoOco")]
	tipo:    String,
	#[sqlx(rename = "Faltas")]
	faltas:  Json<FaltasTabela>,
	#[sqlx(rename = "CondPre")]
	condpre: Json<CondTabela>,
	#[sqlx(rename = "CondPos")]
	condpos: Json<CondTabela>,
}

impl Ocorrencia{
	pub fn calculate_duration(&self)->String{
		let duration = self.fim - self.inicio;
		let fmt_duration = format!("{}ms", chrono::NaiveTime::from_hms(0, 0, 0) + duration);
		fmt_duration
	}
	pub fn make_title(&self)->String{
		format!("{}:{} {} {}",
			self.tipo,
			self.se,
			self.al,
			self.eqp,
		)
	}
	pub fn id(&self)->i32{
		self.id
	}
	pub fn eqp(&self)->(&str,&str,&str){
		(&self.se, &self.al, &self.eqp)
	}
}


///vai para o email
#[derive(sqlx::FromRow,Serialize)]
pub struct OcorrenciaSoe{
	#[sqlx(rename = "EventTime")]
	#[serde(serialize_with = "date_to_str")]
	inicio:   NaiveDateTime,
	#[sqlx(rename = "E3TimeStamp")]
	#[serde(serialize_with = "date_to_str")]
	fim:      NaiveDateTime,
	#[sqlx(rename = "Mensagem")]
	mensagem: Option<String>,
	#[sqlx(rename = "ActorID")]
	agente:   Option<String>
}
impl OcorrenciaSoe{
	pub fn new(inicio:NaiveDateTime,fim:NaiveDateTime,mensagem:Option<String>,agente:Option<String>) -> Self{
		OcorrenciaSoe { inicio, fim, mensagem, agente }
	}
}

///vai para o email
#[derive(sqlx::FromRow,Serialize)]
pub struct PrevEqp{
	#[sqlx(rename = "Faltas")]
	faltas:   Json<FaltasTabela>,
	#[sqlx(rename = "DtHrOco")]
	#[serde(serialize_with = "date_to_str")]
	inicio:   NaiveDateTime,
	#[sqlx(rename = "ProtSen")]
	prot_sen: Option<String>,
	#[sqlx(rename = "ProtAtu")]
	prot_atu: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FaltasTabela{
	IaF: f32,
	IbF: f32,
	IcF: f32,
	InF: f32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CondTabela{
	P: f32,
	Ia: f32,
	Ib: f32,
	Ic: f32,
	In: f32,
}

fn date_to_str<S>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where 
	S: serde::ser::Serializer,
{
	serializer.serialize_str(&time.format("%H:%M:%S  %d-%m-%Y").to_string())
}

//TODO: ver como serde deserializa Option<String>
#[allow(dead_code)]
fn maybe_str<S>(str: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where 
	S: serde::ser::Serializer,
{
	serializer.serialize_str(str.as_ref().map(|w|w.as_str()).unwrap_or(""))
}