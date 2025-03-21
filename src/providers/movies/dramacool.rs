use crate::{html::movies::dramacool_html::DramaCoolHTML, CLIENT};

use futures::{stream::FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Contains all the DramaCool Info
pub struct DramaCool;

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub other_names: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolSearchResults {
    pub current_page: usize,
    pub has_next_page: bool,
    pub total_pages: usize,
    pub total_results: usize,
    pub results: Vec<DramaCoolResult>,
}

const BASE_URL: &'static str = "https://dramacool.com.pa";

impl DramaCool {
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<DramaCoolSearchResults> {
        let current_page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = CLIENT
            .get(format!(
                "{}/search?keyword={}&page={}",
                BASE_URL, parsed_query, current_page
            ))
            .send()
            .await?
            .text()
            .await?;

        let (ids, has_next_page, total_pages) = self.parse_search(&page_html);

        let urls: Arc<Vec<String>> = Arc::new(
            ids.iter()
                .map(|id| format!("{}/{}", BASE_URL, id))
                .collect(),
        );

        let bodies = urls
            .iter()
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;

                async move {
                    let resp = client.get(url).send().await;
                    match resp {
                        Ok(response) => {
                            let text = response.text().await;
                            text.map(|body| (index, body))
                                .map_err(|e| format!("Failed to fetch body: {}", e))
                        }
                        Err(e) => Err(format!("Failed to fetch URL: {}", e)),
                    }
                }
            })
            .collect::<FuturesUnordered<_>>();

        let results: Arc<Mutex<Vec<DramaCoolResult>>> = Arc::new(Mutex::new(vec![]));

        bodies
            .for_each(|result| {
                let urls = Arc::clone(&urls);
                let results = Arc::clone(&results);

                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];

                            let result = self.single_page(&text, id, url);

                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(DramaCoolSearchResults {
            current_page,
            has_next_page,
            total_pages,
            total_results: results.len(),
            results,
        })
    }
}
