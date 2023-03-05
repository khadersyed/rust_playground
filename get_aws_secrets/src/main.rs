use aws_sdk_secretsmanager::{output::GetSecretValueOutput, Client, Error};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct SFTPCred {
    host: String,
    port: String,
    username: String,
    password: Option<String>,
    sftp_path: Option<String>,
}

async fn get_secret(client: &Client, name: &str) -> GetSecretValueOutput {
    let resp = client
        .get_secret_value()
        .secret_id(name)
        .send()
        .await
        .expect("Unable to access secret");
    resp
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let secrets_path = env::var("SFTP_CREDENTIALS_PATH").expect("SFTP_CREDENTIALS_PATH is not set");
    let aws_config = aws_config::load_from_env().await;
    let secrets_client = Client::new(&aws_config);

    let secret_data = get_secret(&secrets_client, &secrets_path).await;

    let sftp_creds: SFTPCred = serde_json::from_str(secret_data.secret_string().unwrap()).unwrap();

    println!("{:?}", sftp_creds);
    Ok(())
}
