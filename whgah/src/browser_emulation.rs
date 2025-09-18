use headless_chrome;
use anyhow;

use headless_chrome::{Browser, LaunchOptionsBuilder};
use anyhow::Result;

pub fn get_page(page: &str) -> Result<String> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .build()?;
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(page)?;
    tab.wait_until_navigated()?;

    // Get the fully-rendered HTML of the page
    let html = tab.get_content()?;
    
    Ok(html)
}