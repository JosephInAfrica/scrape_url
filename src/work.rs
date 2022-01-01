use std::fs;

pub fn do_work(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let url = "http://www.rust-lang.org";
    let output = "rust.md";
    println!("fetching url {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;

    println!("converted saved to {}", output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let url = "http://www.rust-lang.org";
        assert_eq!(do_work(url), Result::Ok(()))
    }
}
