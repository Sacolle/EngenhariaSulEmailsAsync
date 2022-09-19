use lettre::transport::smtp::{authentication::Credentials};
use lettre::{Message, AsyncSmtpTransport, AsyncTransport,Tokio1Executor,
	message::{header,MultiPart,SinglePart,Mailbox}
};
use crate::error::{MissignFieldError,TableProcessError};

use crate::INIVALS;

pub async fn send_email(destinos:&Vec<(Option<String>,Option<String>)>,
	title:String, html:String) -> Result<(),TableProcessError>
{
	let mut email = Message::builder()
		.from(format!("{} <{}>",
			&INIVALS.email_nome,
			&INIVALS.email_addrs)
			.parse()?
		)
		.subject(title);

	//adiciona todos os destinos ao email
	for mbox in destinos{
		email = email.to(Mailbox::new(
			mbox.0.clone(),
			mbox.1.as_ref()
				.ok_or(MissignFieldError::new("email_addrs"))?
				.parse()?
			));
	}
	//compoem-se o email
	let email = email.multipart(
			MultiPart::alternative() //para essa lib, para mandar um email com html deve-se ter um fallback 
			.singlepart(
				SinglePart::builder()
					.header(header::ContentType::TEXT_PLAIN)
					.body(String::from("Falha em carregar a tabela")), //fallback.
			)
			.singlepart(
				SinglePart::builder()
					.header(header::ContentType::TEXT_HTML)
					.body(html),
			),
		)?;

	let creds = Credentials::new(
		INIVALS.email_addrs.clone(),
		INIVALS.email_senha.clone()
	);

	// Open a remote connection to gmail
	let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.office365.com")?
		.credentials(creds)
		.build();

	// Send the email
	mailer.send(email).await?;
	Ok(())
}