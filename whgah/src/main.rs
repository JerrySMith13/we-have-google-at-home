mod site_indexing;
mod log;
mod browser_emulation;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let url = &args[1];

    let all_links = site_indexing::index_site(url);
    
    loop{
        if let Ok(r) = all_links.try_read(){
            for link in r.iter(){
                println!("{}", link);
            }
        }
    }

}
