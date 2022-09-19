use configparser::ini::Ini;
use std::collections::HashMap;

static MARIA:&str = "chaves_db_maria";
static ORACLE:&str = "chaves_db_oracle";
static EMAIL:&str = "email_creds";

///carrega as infos do arquivo ini a u hashmap usando a lib configparser
pub fn load_config(file:&str)->HashMap<&'static str, String>{
	let mut ini = Ini::new();
	ini.load(file).unwrap();

	let mut res = HashMap::new();

	//chaves para a maria db
	let maria_url = format!("mysql://{}:{}@{}/",
		ini.get(MARIA,"user").unwrap(),
		ini.get(MARIA,"senha").unwrap(),
		ini.get(MARIA,"url").unwrap()
	);		
	res.insert("maria_url", maria_url);
	res.insert("maria_emaildb", ini.get(MARIA, "emaildb").unwrap());

	//chaves para o oracle db
	let oracle_url = format!("{}:{}",
		ini.get(ORACLE,"url").unwrap(),
		ini.get(ORACLE,"port").unwrap(),
	);
	res.insert("oracle_url",oracle_url);
	res.insert("oracle_user", ini.get(ORACLE,"user").unwrap());
	res.insert("oracle_senha", ini.get(ORACLE,"senha").unwrap());

	//chaves para o email
	res.insert("email_nome", ini.get(EMAIL,"nome").unwrap());
	res.insert("email_addrs", ini.get(EMAIL,"email").unwrap());
	res.insert("email_senha", ini.get(EMAIL,"senha").unwrap());

	res
}
