use std::{collections::HashSet, sync::{RwLock, Arc}};
use html_parser::{self, Node};
use std::thread;
use crate::log;


fn get_indexes(found: Arc<RwLock<HashSet<String>>>, link: String, root: String) -> Option<()>{
    
    let page = get_page(link.as_str())?;
    let page = html_parser::Dom::parse(&page);

    if page.is_err(){
        return None;
    }
    let page = page.unwrap();
    for child in page.children{
        get_all_from_node(&child, found.clone(), root.clone());
    }

    Some(())
}

fn get_all_from_node(root: &Node, found: Arc<RwLock<HashSet<String>>>, begins_with: String){
    
    if let Some(element) = root.element(){
        let mut own_links = Vec::with_capacity(element.children.len());
        if element.name.to_lowercase() == "a"{
            if let Some(href) = element.attributes.get("href"){
                    if let Some(href) = href{
                        if href.starts_with(begins_with.as_str()){
                            if !found.read().unwrap().contains(href){
                                own_links.push(href.clone());
                                let links_ref = found.clone();
                                let href = href.clone();
                                let root = begins_with;
                                thread::spawn(move || get_indexes(links_ref, href, root));
                                todo!("Add thread pool to check state of all threads");
                            }
                        }
                    }
                }
        }
        if element.children.len() > 0{
            for i in 0..element.children.len(){
                get_all_from_node(&element.children[i], found.clone(), begins_with.clone());
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
    let response = reqwest::blocking::get(url).ok()?;
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
    get_indexes(all_links.clone(), url.to_string(), root.to_string());
    return all_links;
}