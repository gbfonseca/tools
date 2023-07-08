use reqwest::{multipart, Client};
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::future::Future;

pub fn deploy() -> impl Future<Output = Result<(), Box<dyn Error>>> {
    async move {
        println!("===============================================================");
        println!("Fazendo upload da pasta server pra criar a função serverless...");
        println!("===============================================================");

        let mut file = File::open("server/developer-code.js")?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let part = multipart::Part::bytes(buffer)
            .file_name("developer-code.js")
            .mime_str("text/plain")?;

        let form = multipart::Form::new()
            .text("slug", "miniapp-zoro")
            .part("file", part);

        let client = Client::new();

        let response = client
            .post("http://localhost:3000/colossus/file")
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Upload concluído com sucesso!");
        } else {
            let error_message = format!("Falha no upload. Código de status: {}", response.status());
            println!("{}", error_message);
        }

        Ok(())
    }
}
