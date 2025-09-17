use std::{collections::{HashSet, VecDeque}, sync::{Arc, RwLock}, thread::JoinHandle};
use html_parser::{self, Node};
use hyper::Method;
use std::thread;
use crate::log;


fn get_indexes(found: Arc<RwLock<HashSet<String>>>, link: String, root: String) -> Option<()>{
    
    let page = get_page(link.as_str()).unwrap();
    let page = html_parser::Dom::parse(&page);

	let pool: VecDeque<JoinHandle<Option<()>>> = VecDeque::with_capacity(100);
	let pool_ref = Arc::new(RwLock::new(pool));

    if page.is_err(){
        return None;
    }
    let page = page.unwrap();
    for child in page.children{
        get_all_from_node(&child, found.clone(), root.clone(), pool_ref.clone());
    }

    Some(())
}

fn get_all_from_node(root: &Node, found: Arc<RwLock<HashSet<String>>>, begins_with: String, thread_pool: Arc<RwLock<VecDeque<JoinHandle<Option<()>>>>>){
    
    if let Some(element) = root.element(){
        let mut own_links = Vec::with_capacity(element.children.len());
        if element.name.to_lowercase() == "a"{
            if let Some(href) = element.attributes.get("href"){
                    if let Some(href) = href{
                        if !found.read().unwrap().contains(href){
                            own_links.push(href.clone());
                            let links_ref = found.clone();
                            let href = href.clone();
                            let root = begins_with.clone();
                            let handle = thread::spawn(move || get_indexes(links_ref, href, root));
							thread_pool.write().unwrap().push_back(handle);
                        } 
                    }
                }
        }
        if element.children.len() > 0{
            println!("Started iterating through children");
            for i in 0..element.children.len(){
                get_all_from_node(&element.children[i], found.clone(), begins_with.clone(), thread_pool.clone());
            }
        }
        found.clear_poison();
        let mut write = found.write().unwrap();
        for link in own_links{
            write.insert(link);
        }
    }
    
}

fn get_page(url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::new().request(Method::GET, url)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
    .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
    .header("Accept-Language", "en-US,en;q=0.9")
    .header("Referer", "https://google.com/");
    let response = client.send().unwrap();
    if response.status() != reqwest::StatusCode::OK {
        return None;
    }
    response.text().ok()
}

pub fn index_site(url: &str) -> Arc<RwLock<HashSet<String>>>{
    let all_links: Arc<RwLock<HashSet<String>>> = Arc::new(
        RwLock::new(HashSet::with_capacity(100))
    );

    let root = url.split_once('/').unwrap_or((url, "")).0;
    get_indexes(all_links.clone(), url.to_string(), root.to_string()).unwrap();
    return all_links;
}