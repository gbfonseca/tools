use clap::Command;
use cmd::deploy::deploy;
mod cmd;

#[tokio::main]
async fn main() {
   let app_matches = Command::new("tools")
        .about("Library to serverless")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("deploy")
            .about("Deploy serverless")
        ).get_matches();

     

    if let Some("deploy") = app_matches.subcommand_name() {
       let _ = deploy().await;
    } else {
        panic!("Erro");
    }
}
