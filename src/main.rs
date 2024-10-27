//RSS is structured like channel, inside which theres title, link, description, etc. just so you know ha



use reqwest::blocking::get;
use rss::Channel; 
use std::error::Error; 


pub fn get_info(url: &str) -> Result<(String, String), Box<dyn Error>> { 
    let response = reqwest::blocking::get(url)?;
    let content = response.bytes()?; 
    let channel = Channel::read_from(&content[..])?;
    //that's to parse the content of the rss feed 

    let title = channel.title().to_string();
    let link = channel.link().to_string();

    Ok((title,link))
}

//now to try it out 

fn main() -> Result<(), Box<dyn Error>> {
    // Test with one feed first
    let test_url = "https://feeds.bbci.co.uk/news/rss.xml";
    println!("Reading...{}", test_url);
    match get_info(test_url) {
    Ok((title, link)) => {
        println!("Feed Title: {}", title);
        println!("Feed Link: {}", link);
    }
    Err(e) => println!("Error: {}", e)
}

Ok(())

}
