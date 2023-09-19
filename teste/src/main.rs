use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
} 

fn main() -> Result<()> {
   let cep = "88054-600";
    let url = format!(
        "https://viacep.com.br/ws/{}/json/", cep);

    let mut res = reqwest
        ::blocking
        ::get(url)?;
        
    println!("Status {}", res.status() );
    println!("Headers \n {:#?}", res.headers());
    res.copy_to(&mut std::io::stdout());
    Ok(())
}
