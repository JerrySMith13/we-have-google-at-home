use std::collections::HashSet;
use html_parser;

use crate::log;


pub fn index(url: &str, pre_alloc: usize) -> Vec<&str>{
    let index_begin: HashSet<&str> = HashSet::with_capacity(pre_alloc);
    

    
}

fn get_all_indexes(url: &str) -> Option<Vec<&str>>{
    let webpage: &str = match get_page(url){
        Some(o) => o,
        None => return None
    };
    //Get all of the urls
    let page = match html_parser::Dom::parse(webpage){
        Ok(page) => page,
        Err(_) => {
            todo!();  
            return None;
        }
    };

    todo!("Add ability to pipe parsed html data into ")


    //Pass each to this same function recursively

    return Some(Vec::new())
}

fn get_page(url: &str) -> Option<&str>{
    todo!()
}