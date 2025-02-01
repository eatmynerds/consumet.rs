use crate::{
    models::types::TvType,
    providers::movies::flixhq::{FlixHQ, FlixHQEpisode, FlixHQResult, FlixHQServer, BASE_URL},
};

use visdom::{types::Elements, Vis};

fn create_html_fragment(html: &str) -> Elements<'_> {
    Vis::load(html).expect("Failed to load html")
}

pub(crate) trait FlixHQHTML {
    fn parse_recent_shows(&self, html: &str) -> Vec<Option<String>>;
    fn parse_recent_movies(&self, html: &str) -> Vec<Option<String>>;
    fn parse_trending_movies(&self, html: &str) -> Vec<Option<String>>;
    fn parse_trending_shows(&self, html: &str) -> Vec<Option<String>>;
    fn parse_search(&self, html: &str) -> (Vec<String>, bool, usize);
    fn single_page(&self, html: &str, id: &str, url: &str) -> FlixHQResult;
    fn info_season(&self, html: &str) -> Vec<String>;
    fn info_episode(&self, html: &str) -> EpisodesInfo;
    fn info_server(&self, html: &str, media_id: &str) -> Vec<FlixHQServer>;
}

impl FlixHQHTML for FlixHQ {
    fn parse_recent_shows(&self, html: &str) -> Vec<Option<String>> {
        let trending_parser = Recent::new(html);

        trending_parser.recent_shows()
    }

    fn parse_recent_movies(&self, html: &str) -> Vec<Option<String>> {
        let trending_parser = Recent::new(html);

        trending_parser.recent_movies()
    }

    fn parse_trending_movies(&self, html: &str) -> Vec<Option<String>> {
        let trending_parser = Trending::new(html);

        trending_parser.trending_movies()
    }

    fn parse_trending_shows(&self, html: &str) -> Vec<Option<String>> {
        let trending_parser = Trending::new(html);

        trending_parser.trending_shows()
    }

    fn parse_search(&self, html: &str) -> (Vec<String>, bool, usize) {
        let page_parser = Page::new(html);

        (
            page_parser.page_ids(),
            page_parser.has_next_page(),
            page_parser.total_pages(),
        )
    }

    fn single_page(&self, html: &str, id: &str, url: &str) -> FlixHQResult {
        let search_parser = Search::new(html);

        let info_parser = Info::new(html);

        FlixHQResult {
            cover: search_parser.cover(),
            title: search_parser.title(),
            url: url.to_string(),
            image: search_parser.image(),
            country: info_parser.label(1, "Country:"),
            genres: info_parser.label(2, "Genre:"),
            release_date: info_parser.label(3, "Released:").join(""),
            media_type: search_parser.media_type(id),
            id: id.to_string(),
            description: info_parser.description(),
            quality: info_parser.quality(),
            rating: info_parser.rating(),
            duration: info_parser.duration(),
            production: info_parser.label(4, "Production:"),
            casts: info_parser.label(5, "Casts:"),
            tags: info_parser.label(6, "Tags:"),
        }
    }

    fn info_season(&self, html: &str) -> Vec<String> {
        let season_parser = Seasons::new(html);

        season_parser
            .season_results()
            .into_iter()
            .flatten()
            .collect()
    }

    fn info_episode(&self, html: &str) -> EpisodesInfo {
        let episode_parser = EpisodesInfo::default();


        episode_parser.episode_results(html)
    }

    fn info_server(&self, html: &str, media_id: &str) -> Vec<FlixHQServer> {
        let server_parser = Server::new(html);

        server_parser.parse_server_html(media_id)
    }
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
        self.elements
        .find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li:nth-child(1)")
        .has_class("active")
    }

    fn total_pages(&self) -> usize {
        let total_pages_attr = self.elements.find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li.page-item:last-child a").attr("href");

        if let Some(total_pages) = total_pages_attr {
            if let Some(pages) = total_pages.to_string().rsplit('=').next() {
                return pages.parse::<usize>().unwrap_or(1);
            }
        }

        1
    }

    fn page_ids(&self) -> Vec<String> {
        self.elements
            .find("div.film-poster > a")
            .into_iter()
            .filter_map(|element| {
                element
                    .get_attribute("href")
                    .and_then(|href| href.to_string().strip_prefix('/').map(String::from))
            })
            .collect()
    }
}

struct Search<'b> {
    elements: Elements<'b>,
}

impl<'b> Search<'b> {
    fn new(html: &'b str) -> Self {
        let elements = create_html_fragment(html);
        Self { elements }
    }

    fn image(&self) -> String {
        let image_attr = self
            .elements
            .find("div.m_i-d-poster > div > img")
            .attr("src");

        if let Some(image) = image_attr {
            return image.to_string();
        };

        String::new()
    }

    fn title(&self) -> String {
        self.elements
        .find(
            "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
        )
        .text()
        .trim()
        .to_owned()
    }

    fn cover(&self) -> String {
        let cover_attr = self.elements.find("div.w_b-cover").attr("style");
        if let Some(cover) = cover_attr {
            return cover
                .to_string()
                .replace("background-image: url(", "")
                .replace(')', "");
        };

        String::new()
    }

    fn media_type(&self, id: &str) -> TvType {
        match id.split('/').next() {
            Some("tv") => TvType::TvSeries,
            Some("movie") => TvType::Movie,
            _ => todo!(),
        }
    }
}

struct Info<'b> {
    elements: Elements<'b>,
}

impl<'b> Info<'b> {
    fn new(html: &'b str) -> Self {
        let elements = create_html_fragment(html);

        Self { elements }
    }

    fn label(&self, index: usize, label: &str) -> Vec<String> {
        self.elements
            .find(&format!(
                "div.m_i-d-content > div.elements > div:nth-child({})",
                index
            ))
            .text()
            .replace(label, "")
            .split(',')
            .map(|s| s.trim().to_owned())
            .collect()
    }

    fn description(&self) -> String {
        self.elements.find("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").text().trim().to_owned()
    }

    fn quality(&self) -> String {
        self.elements
            .find("span.item:nth-child(1)")
            .text()
            .trim()
            .to_owned()
    }

    fn rating(&self) -> String {
        self.elements
            .find("span.item:nth-child(2)")
            .text()
            .trim()
            .to_owned()
    }

    fn duration(&self) -> String {
        self.elements
            .find("span.item:nth-child(3)")
            .text()
            .trim()
            .to_owned()
    }
}



struct Seasons<'a> {
    elements: Elements<'a>,
}

impl<'a> Seasons<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);

        Self { elements }
    }

    fn season_results(&self) -> Vec<Option<String>> {
        self.elements.find(".dropdown-menu > a").map(|_, element| {
            element
                .get_attribute("data-id")
                .map(|value| value.to_string())
        })
    }
}

pub struct Episodes<'a> {
    elements: Elements<'a>,
}

#[derive(Default)]
pub struct EpisodesInfo {
    pub episodes: Vec<FlixHQEpisode>,
    index: usize,
}

impl Iterator for EpisodesInfo {
    type Item = FlixHQEpisode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.episodes.len() {
            let episode = self.episodes[self.index].clone();
            self.index += 1;
            Some(episode)
        } else {
            None
        }
    }
}

impl<'a> Episodes<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);
        Self { elements }
    }

    fn episode_title(&self) -> Vec<Option<String>> {
        self.elements.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("title")
                .map(|value| value.to_string())
        })
    }

    fn episode_id(&self) -> Vec<Option<String>> {
        self.elements.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("data-id")
                .map(|value| value.to_string())
        })
    }
}

impl EpisodesInfo {
    fn episode_results(&self, html: &str) -> EpisodesInfo {
        let episode_info_parser = Episodes::new(html);
        let episode_titles = episode_info_parser.episode_title();
        let episode_ids = episode_info_parser.episode_id();

        let episode: Vec<FlixHQEpisode> = episode_ids
            .iter()
            .zip(episode_titles.iter())
            .flat_map(|(id, title)| id.as_ref().map(|id| (id, title)))
            .map(|(id, title)| {
                let url = format!("{}/ajax/v2/episode/servers/{}", BASE_URL, id);
                FlixHQEpisode {
                    id: id.clone(),
                    title: title.clone().unwrap_or(String::from("")),
                    url,
                }
            })
            .collect();

        EpisodesInfo {
            episodes: episode,
            index: 0,
        }
    }
}

struct Server<'a> {
    elements: Elements<'a>,
}

impl<'a> Server<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);

        Self { elements }
    }

    fn parse_server_html(&self, media_id: &str) -> Vec<FlixHQServer> {
        self.elements.find("ul > li > a").map(|_, element| {
            let id = element
                .get_attribute("id")
                .map(|value| value.to_string().replace("watch-", ""))
                .unwrap_or(String::from(""));

            let name = element
                .get_attribute("title")
                .map(|value| value.to_string().trim_start_matches("Server ").to_owned());

            let url = format!("{}/watch-{}.{}", BASE_URL, media_id, id);
            let name = name.unwrap_or(String::from(""));

            FlixHQServer { name, url }
        })
    }
}

struct Recent<'a> {
    elements: Elements<'a>,
}

impl<'a> Recent<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);

        Self { elements }
    }

    fn recent_movies(&self) -> Vec<Option<String>> {
        self.elements.find("#main-wrapper > div > section:nth-child(6) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a").map(|_, element| {
            element
                .get_attribute("href")?
                .to_string()
                .strip_prefix('/')
                .map(String::from)
        })
    }

    fn recent_shows(&self) -> Vec<Option<String>> {
        self.elements.find("#main-wrapper > div > section:nth-child(7) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a").map(|_, element| {
            element
                 .get_attribute("href")?
                .to_string()
                .strip_prefix('/')
                .map(String::from)

        })
    }
}

struct Trending<'a> {
    elements: Elements<'a>,
}

impl<'a> Trending<'a> {
    fn new(html: &'a str) -> Self {
        let elements = create_html_fragment(html);
        Self { elements }
    }

    fn trending_movies(&self) -> Vec<Option<String>> {
        self.elements
            .find("div#trending-movies div.film_list-wrap div.flw-item div.film-poster a")
            .map(|_, element| {
                element
                    .get_attribute("href")?
                    .to_string()
                    .strip_prefix('/')
                    .map(String::from)
            })
    }

    fn trending_shows(&self) -> Vec<Option<String>> {
        self.elements
            .find("div#trending-tv div.film_list-wrap div.flw-item div.film-poster a")
            .map(|_, element| {
                element
                    .get_attribute("href")?
                    .to_string()
                    .strip_prefix('/')
                    .map(String::from)
            })
    }
}
