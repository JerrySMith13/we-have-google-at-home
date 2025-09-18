use std::{collections::{HashSet, VecDeque}, str::FromStr, sync::{Arc, RwLock}, thread::JoinHandle};
use html_parser::{self, Node};
use reqwest::Url;
use std::thread;
use crate::log;

use crate::browser_emulation::get_page;

fn get_indexes(found: Arc<RwLock<HashSet<String>>>, link: String) -> Option<()>{
    
    let page = get_page(link.as_str()).unwrap();
    let page = html_parser::Dom::parse(&page);
	

	let pool: VecDeque<JoinHandle<Option<()>>> = VecDeque::with_capacity(100);
	let pool_ref = Arc::new(RwLock::new(pool));

    if page.is_err(){
        return None;
    }
    let page = page.unwrap();
    for child in page.children{
        get_all_from_node(&child, found.clone(), pool_ref.clone(), Url::from_str(&link.clone()).unwrap());
    }

    Some(())
}

fn get_all_from_node(root: &Node, found: Arc<RwLock<HashSet<String>>>, thread_pool: Arc<RwLock<VecDeque<JoinHandle<Option<()>>>>>, link: Url){
    
    if let Some(element) = root.element(){
		
        let mut own_links = Vec::with_capacity(element.children.len());
        if element.name.to_lowercase() == "a"{
			
            if let Some(href) = element.attributes.get("href"){
                    if let Some(href) = href{
						
						let url = Url::from_str(href).unwrap();
						if let Some(host) = url.host_str(){
							
							if host == link.host_str().unwrap(){
								
								if !found.read().unwrap().contains(href){
                            		own_links.push(href.clone());
                            		let links_ref = found.clone();
                            		let href = href.clone();
                            		let handle = thread::spawn(move || get_indexes(links_ref, href));
									thread_pool.write().unwrap().push_back(handle);
                        		} 
							}
						}
						else {
							let href = canonicalize(link.as_str(), &href).unwrap();
							
							let links_ref = found.clone();
                            let handle = thread::spawn(move || get_indexes(links_ref, href));
							thread_pool.write().unwrap().push_back(handle);
						}
                        
                    }
                }
        }
        if element.children.len() > 0{
            println!("Started iterating through children");
            for i in 0..element.children.len(){
                get_all_from_node(&element.children[i], found.clone(), thread_pool.clone(), link.clone());
            }
        }
        found.clear_poison();
        let mut write = found.write().unwrap();
        for link in own_links{
            write.insert(link);
        }
    }
    
}


fn canonicalize(base: &str, relative: &str) -> Option<String>{
	let base = Url::parse(base).ok()?;
    let resolved = base.join(relative).ok()?;
    Some(resolved.into())
}
pub fn index_site(url: &str) -> Arc<RwLock<HashSet<String>>>{
    let all_links: Arc<RwLock<HashSet<String>>> = Arc::new(
        RwLock::new(HashSet::with_capacity(100))
    );

	let url  = Url::from_str(url).unwrap();

    get_indexes(all_links.clone(), url.to_string()).unwrap();
    return all_links;
}