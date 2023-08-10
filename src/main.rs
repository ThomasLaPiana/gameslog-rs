use reqwest;

fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::blocking::get("https://ign.com")?;

    println!("Status Code: {:?}", response.status());
    println!("Path: {:?}", response.url().path());

    Ok(())
}
