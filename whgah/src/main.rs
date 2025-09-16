mod site_indexing;
mod log;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let url = &args[1];

    let all_links = site_indexing::index_site(url);
    let links_read = all_links.read().unwrap();
    for link in links_read.iter() {
        println!("{link}");
    }

}
