use configparser::ini::Ini;

static MARIA:&str = "chaves_db_maria";
static ORACLE:&str = "chaves_db_oracle";
static EMAIL:&str = "email_creds";

pub struct IniVals{
	pub maria_url: String,
	pub maria_emaildb: String,
	pub oracle_url: String,
	pub oracle_port: String,
	pub oracle_user: String,
	pub oracle_senha: String,
	pub email_nome: String,
	pub email_addrs: String,
	pub email_senha: String,
}

///carrega as infos do arquivo ini a u hashmap usando a lib configparser
pub fn load_config(file:&str)->IniVals{
	let mut ini = Ini::new();
	ini.load(file).unwrap();

	//chaves para a maria db
	let maria_url = format!("mysql://{}:{}@{}/",
		ini.get(MARIA,"user").unwrap(),
		ini.get(MARIA,"senha").unwrap(),
		ini.get(MARIA,"url").unwrap()
	);		
	let maria_emaildb = ini.get(MARIA, "emaildb").unwrap();

	//chaves para o oracle db
	let oracle_url = ini.get(ORACLE,"url").unwrap();
	let oracle_port = ini.get(ORACLE,"port").unwrap();
	let oracle_user = ini.get(ORACLE,"user").unwrap();
	let oracle_senha = ini.get(ORACLE,"senha").unwrap();

	//chaves para o email
	let email_nome = ini.get(EMAIL,"nome").unwrap();
	let email_addrs = ini.get(EMAIL,"email").unwrap();
	let email_senha = ini.get(EMAIL,"senha").unwrap();

	IniVals { maria_url, maria_emaildb, oracle_url, oracle_port, oracle_user, oracle_senha, email_nome, email_addrs, email_senha }
}
