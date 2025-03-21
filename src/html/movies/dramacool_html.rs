use visdom::{types::Elements, Vis};

use crate::providers::movies::dramacool::{DramaCool, DramaCoolResult};

pub trait DramaCoolHTML {
    fn parse_search(&self, html: &str) -> (Vec<String>, bool, usize);
    fn single_page(&self, html: &str, id: &str, url: &str) -> DramaCoolResult;
}

impl DramaCoolHTML for DramaCool {
    fn parse_search(&self, html: &str) -> (Vec<String>, bool, usize) {
        let page_parser = Page::new(html);

        (
            page_parser.page_ids(),
            page_parser.has_next_page(),
            page_parser.total_pages(),
        )
    }

    fn single_page(&self, html: &str, id: &str, url: &str) -> DramaCoolResult {
        let fragment = create_html_fragment(html);

        let search_parser = Search {
            elements: &fragment,
            id,
        };

        DramaCoolResult {
            title: search_parser.title(),
            other_names: search_parser.other_names(),
            url: url.to_string(),
            image: search_parser.image(),
            release_date: search_parser.release_date(),
            id: id.to_string(),
        }
    }
}

fn create_html_fragment(page_html: &str) -> Elements<'_> {
    Vis::load(page_html).unwrap()
}

struct Page<'a> {
    elements: Elements<'a>,
}

impl<'a> Page<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);

        Self { elements }
    }

    fn has_next_page(&self) -> bool {
        self.elements.find("ul.pagination li").has_class("selected")
    }

    fn total_pages(&self) -> usize {
        let total_pages_href = self
            .elements
            .find("ul.pagination li.last:last-child a")
            .attr("href");

        if let Some(page_href) = total_pages_href {
            if let Some(pages) = page_href.to_string().rsplit('=').next() {
                return pages.parse::<usize>().unwrap_or(1);
            }
        }

        1
    }

    fn page_ids(&self) -> Vec<String> {
        self.elements
            .find("div.block div.tab-content ul.list-episode-item li a")
            .into_iter()
            .filter_map(|element| {
                element
                    .get_attribute("href")
                    .and_then(|href| href.to_string().strip_prefix('/').map(String::from))
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Search<'page, 'b> {
    elements: &'b Elements<'page>,
    id: &'b str,
}

impl<'page, 'b> Search<'page, 'b> {
    fn title(&self) -> String {
        match self.id.split('/').last() {
            Some(title) => title.to_owned(),
            None => String::new(),
        }
    }

    fn image(&self) -> String {
        let image_attr = self.elements.find("div.details div.img img").attr("src");

        if let Some(image) = image_attr {
            return image.to_string();
        };

        String::new()
    }

    fn release_date(&self) -> String {
        self.elements
            .find(r#"div.details div.info p:contains("Released:")"#)
            .text()
            .replace("Released:", "")
            .trim()
            .to_owned()
    }

    fn other_names(&self) -> Vec<String> {
        self.elements
            .find(".other_name > a")
            .map(|_, element| element.text().trim().to_owned())
    }
}
