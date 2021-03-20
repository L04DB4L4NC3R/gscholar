extern crate reqwest;
extern crate select;

use scraper::{Html, Selector};

pub struct Client {
     client: reqwest::Client,
}

pub enum Error {
    ConnectionError,
    ParseError,
    InvalidServiceError,
    RequiredFieldError,
    NotImplementedError,
    InvalidResponseError,
}

pub struct ScholarResult {
   pub title: String,
   pub author: String,
   pub abs: String,
   pub link: String,
}

pub struct ScholarArgs {
   // q - required
   pub query: &'static str,

   // cites - citaction id to trigger "cited by"
   pub cite_id: Option<&'static str>,

   // as_ylo - give results from this year onwards
   pub from_year: Option<u16>,

   // as_yhi
   pub to_year: Option<u16>,

   // scisbd - 0 for relevence, 1 to include only abstracts, 2 for everything. Default = date
   pub sort_by: Option<u8>,

   // cluster - query all versions. Use with q and cites prohibited
   pub cluster_id: Option<&'static str>,

   // hl - eg: hl=en for english
   pub lang: Option<&'static str>,

   // lr - one or multiple languages to limit the results to
   // eg: lr=lang_fr|lang_en
   pub lang_limit: Option<&'static str>,

   // num - max number of results to return
   pub limit: Option<u32>,

   // start - result offset. Can be used with limit for pagination
   pub offset: Option<u32>,

   // safe - level of filtering
   // safe=active or safe=off
   pub adult_filtering: Option<bool>,

   // filter - whether to give similar/ommitted results
   // filter=1 for similar results and 0 for ommitted
   pub include_similar_results: Option<bool>,

   // as_vis - set to 1 for including citations, otherwise 0
   pub include_citations: Option<bool>,
}

pub trait Args {
    fn get_service(&self) -> Services;
    fn get_url(&self) -> Result<String, Error>;
    fn get_limit(&self) -> usize;
}

impl Args for ScholarArgs {
    fn get_service(&self) -> Services {
        return Services::Scholar;
    }

    fn get_limit(&self) -> usize {
        if let Some(s) = self.limit {
            return s as usize
        }
        return 0usize
    }

    fn get_url(&self) -> Result<String, Error> {
       let mut url = String::from(
           get_base_url(self.get_service())
        );

       if self.query == "" {
           return Err(Error::RequiredFieldError);
       }

       url.push_str("q=");
       url.push_str(self.query);

       if let Some(i) = self.cite_id {
           url.push_str("&cites=");
           url.push_str(i);
       }
       if let Some(i) = self.from_year {
           url.push_str("&as_ylo=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.to_year {
           url.push_str("&as_yhi=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.sort_by {
           if i < 3 {
               url.push_str("&scisbd=");
               url.push_str(&i.to_string()[..]);
           }
       }
       if let Some(i) = self.cluster_id {
           url.push_str("&cluster=");
           url.push_str(i);
       }
       if let Some(i) = self.lang {
           // TODO: validation
           url.push_str("&hl=");
           url.push_str(i);
       }
       if let Some(i) = self.lang_limit {
           // TODO: validation
           url.push_str("&lr=");
           url.push_str(i);
       }
       if let Some(i) = self.limit {
           url.push_str("&num=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.offset {
           url.push_str("&start=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.adult_filtering {
           url.push_str("&safe=");
           if i {
               url.push_str("active");
           } else {
               url.push_str("off");
           }
       }
       if let Some(i) = self.include_similar_results {
           url.push_str("&filter=");
           if i {
               url.push_str("1");
           } else {
               url.push_str("0");
           }
       }
       if let Some(i) = self.include_citations {
           url.push_str("&as_vis=");
           if i {
               url.push_str("1");
           } else {
               url.push_str("0");
           }
       }
       return Ok(url);
    }
}

pub enum Services {
    Scholar,
}

pub fn init_client() -> Client {
    let client = reqwest::Client::new();
    Client{client}
}

fn get_base_url<'a>(service: Services) -> &'a str {
    match service {
        Services::Scholar => "https://scholar.google.com/scholar?",
    }
}

impl Client {
    async fn get_document(&self, url: &str) -> Result<String, errors::Error> {
        let resp = self.client.get(url)
            .send()
            .await;
        if !resp.is_ok() {
            return Err(Error::ConnectionError);
        }
        let val: String = resp.unwrap().text().await.unwrap();
        return Ok(val);
    }

    fn scrape_serialize(&self, document: String) -> Result<Vec<ScholarResult>, Error> {
        let fragment = Html::parse_document(&document[..]);

        let article_selector = Selector::parse(".gs_ri").unwrap();
        let title_selector = Selector::parse(".gs_rt").unwrap();
        let abstract_selector = Selector::parse(".gs_rs").unwrap();
        let author_selector = Selector::parse(".gs_a").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        let nodes = fragment.select(&article_selector).collect::<Vec<_>>();

        let response = nodes
            .chunks_exact(1)
            .map(|rows| {
                let title = rows[0].select(&title_selector)
                    .next()
                    .unwrap();
                let link = rows[0].select(&link_selector)
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .unwrap();
                let abs = rows[0].select(&abstract_selector)
                    .next()
                    .unwrap();
                let author = rows[0].select(&author_selector)
                    .next()
                    .unwrap();

                let ti = title.text().collect::<String>();
                let ab = abs.text().collect::<String>();
                let au = author.text().collect::<String>();
                let li = link.to_string();

                let l = ScholarResult{
                    title: ti,
                    author: au,
                    abs: ab,
                    link: li,
                };
                l
            }).collect::<Vec<ScholarResult>>();

        return Ok(response);
    }

    pub async fn scrape_scholar(&self, args: &dyn Args) -> Result<Vec<ScholarResult>, Error> {
        let url: String;
        match args.get_url() {
            Ok(u) => url = u,
            Err(e) => return Err(e),
        };
        
        let doc: String;
        match self.get_document(&url[..]).await {
            Ok(page) => doc = page,
            Err(e) => return Err(e),
        };

        match self.scrape_serialize(doc) {
            Ok(result) => return Ok(result),
            Err(e) => return Err(e),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_query() {
        let sc = ScholarArgs{
            query: "abcd",
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: None,
            cluster_id: None,
            lang: None,
            lang_limit: None,
            limit: None,
            offset: None,
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };

        match sc.get_url() {
            Ok(url) => assert!(url.eq("https://scholar.google.com/scholar?q=abcd"), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }

    #[test]
    fn build_url_all() {
        let sc = ScholarArgs{
            query: "abcd",
            cite_id: Some("213123123123"),
            from_year: Some(2018),
            to_year: Some(2021),
            sort_by: Some(0),
            cluster_id: Some("3121312312"),
            lang: Some("en"),
            lang_limit: Some("lang_fr|lang_en"),
            limit: Some(10),
            offset: Some(5),
            adult_filtering: Some(true),
            include_similar_results: Some(true),
            include_citations: Some(true),
        };
        match sc.get_url() {
            Ok(url) => assert!(
                url.eq("https://scholar.google.com/scholar?q=abcd&cites=213123123123&as_ylo=2018&as_yhi=2021&scisbd=0&cluster=3121312312&hl=en&lr=lang_fr|lang_en&num=10&start=5&safe=active&filter=1&as_vis=1"), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }

    #[tokio::test]
    async fn scrape_with_query() {
        let sc = ScholarArgs{
            query: "machine-learning",
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: None,
            cluster_id: None,
            lang: None,
            lang_limit: None,
            limit: Some(3),
            offset: Some(0),
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };
match sc.get_url() {
            Ok(url) => println!("_URLS {}", url),
            Err(_e) => assert_eq!(false, true),
        }

        let client = init_client();
        match client.scrape_scholar(&sc).await {
            Ok(res) => assert_eq!(res.len(), 3),
            Err(_e) => assert_eq!(true, false),
        }
    }
}
