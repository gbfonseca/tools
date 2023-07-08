use reqwest::{multipart, Client};
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::fs;

pub async fn deploy() -> Result<(), Box<dyn Error>> {
        println!("===============================================================");
        println!("Fazendo upload da pasta server pra criar a função serverless...");
        println!("===============================================================");

        let mut form: multipart::Form = multipart::Form::new();

        for file in fs::read_dir("./server").unwrap() {
            let wrapped_file =file.unwrap();
            let mut opened_file = File::open(wrapped_file.path().display().to_string())?;

            let mut buffer = Vec::new();

            opened_file.read_to_end(&mut buffer)?;
            let file_name = wrapped_file.file_name().into_string().unwrap().to_string();
            let part = multipart::Part::bytes(buffer)
                .file_name(file_name)
                .mime_str("text/plain")?;
            
            
            form = form.part("file", part);
        }

        let form = form
            .text("slug", "miniapp-zoro");

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
            println!("{}", response.text().await.unwrap().to_string());
            println!("{}", error_message);
        }

        Ok(())
    
}
