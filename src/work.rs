use std::fs;

pub fn do_work() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://www.rust-lang.org";
    let output = "rust.md";
    println!("fetching url {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;

    println!("converted saved to {}", output);
    Ok(())
}
