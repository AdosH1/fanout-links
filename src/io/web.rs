pub async fn get_web_body(url: String) -> Result<String, reqwest::Error> {
    let url_response = reqwest::get(url).await;
    match url_response {
        Ok(response) => response.text().await,
        Err(e) => Err(e),
    }
}

pub fn open_links(links: Vec<&str>) {
    for link in links {
        match open::that(link) {
            Ok(_) => {}
            Err(e) => {
                println!("Something went wrong: {}", e)
            }
        }
    }
}
