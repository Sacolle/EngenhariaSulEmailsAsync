use std::{fmt,error};


#[derive(Debug)]
pub enum TableProcessError{
	MissingData(MissignFieldError),
	EmailFormError(lettre::error::Error),
	EmailSendError(lettre::transport::smtp::Error),
	EmailParseError(lettre::address::AddressError),
	TemplatingError(tera::Error),
	OracleDbError(oracle::Error),
	TokioIoError(tokio::io::Error),
	TokioJoinError(tokio::task::JoinError),
	SqlxError(sqlx::Error)
}

impl fmt::Display for TableProcessError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self{
			Self::MissingData(e)=> write!(f,"{}",e),
			Self::EmailFormError(e)=>write!(f,"Email form is configured incorrectly:\n\t{}",e),
			Self::EmailSendError(e)=>write!(f,"Failed to send email:\n\t{}",e),
			Self::EmailParseError(e) => write!(f,"Email Adress is malformed:\n\t{}",e),
			Self::TemplatingError(e)=> write!(f,"Falha no processamento do template:\n\t{}",e),
			Self::OracleDbError(e)=> write!(f,"Falha na Oracle Db:\n\t{}",e),
			Self::TokioIoError(e)=> write!(f,"Falha na escrita dos arquvios:\n\t{}",e),
			Self::TokioJoinError(e)=> write!(f,"Na junção das tasks:\n\t{}",e),
			Self::SqlxError(e)=> write!(f,"Falha em Sqlx:\n\t{}",e),
		}
	}
}

#[derive(Debug)]
pub struct MissignFieldError(pub String);

impl MissignFieldError{
	pub fn new(w:&str)->Self{
		MissignFieldError(String::from(w))
	}
}

impl fmt::Display for MissignFieldError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"Missing field {}",self.0)
	}
}
impl error::Error for MissignFieldError{}

impl From<MissignFieldError> for TableProcessError{
	fn from(e: MissignFieldError) -> Self {
		TableProcessError::MissingData(e)
	}
}

impl From<lettre::error::Error> for TableProcessError{
	fn from(e: lettre::error::Error) -> Self {
		TableProcessError::EmailFormError(e)
	}
}

impl From<lettre::transport::smtp::Error> for TableProcessError{
	fn from(e: lettre::transport::smtp::Error) -> Self {
		TableProcessError::EmailSendError(e)
	}
}

impl From<lettre::address::AddressError> for TableProcessError{
	fn from(e: lettre::address::AddressError) -> Self {
		TableProcessError::EmailParseError(e)
	}
}

impl From<tera::Error> for TableProcessError{
	fn from(e: tera::Error) -> Self {
		TableProcessError::TemplatingError(e)
	}
}

impl From<oracle::Error> for TableProcessError{
	fn from(e: oracle::Error) -> Self {
		TableProcessError::OracleDbError(e)
	}
}

impl From<tokio::io::Error> for TableProcessError{
	fn from(e: tokio::io::Error) -> Self {
		TableProcessError::TokioIoError(e)
	}
}

impl From<tokio::task::JoinError> for TableProcessError{
	fn from(e: tokio::task::JoinError) -> Self {
		TableProcessError::TokioJoinError(e)
	}
}

impl From<sqlx::Error> for TableProcessError{
	fn from(e: sqlx::Error) -> Self {
		TableProcessError::SqlxError(e)
	}
}