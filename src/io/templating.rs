use std::collections::HashMap;
use tera::Context;

use crate::TEMPLATE;
use crate::db::chunks::{Ocorrencia,OcorrenciaSoe,PrevEqp};
use crate::error::TableProcessError;

//função para o sistema de templating tera que permite invocar funcionalidades dentro do template em html
//recebe uma float e o retorna com n casas decimais 
pub fn tera_round_n_places(args:&HashMap<String,tera::Value>)->tera::Result<tera::Value>{
	let res = match args.get("num"){
		Some(num) => args.get("n")
			.map(|n|format!("{1:.0$}",n.as_i64().unwrap() as usize, num.as_f64().unwrap())),
		None => None
	};
	match res.map(|num|tera::to_value(num).map_err(|e|tera::Error::json(e))){
		Some(res) => res,
		None => Err("missing vals".into())
	}
}

pub fn build_from_template(empresa:&str, ocor: Ocorrencia,
	soes: Vec<OcorrenciaSoe>, eqps: Vec<PrevEqp>)->Result<(String,String),TableProcessError>
{
	let mut ctx = Context::new();

	let title = ocor.make_title();

	ctx.insert("empresa",empresa);
	ctx.insert("duracao", &ocor.calculate_duration());
	ctx.insert("ocor", &ocor);
	ctx.insert("vec_soe", &soes);
	ctx.insert("vec_eqp", &eqps);

	Ok((title,TEMPLATE.render("base.html", &ctx)?))
}


