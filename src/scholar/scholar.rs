struct Client {}

struct ScholarResult {}

pub struct ScholarArgs {
   // q - required
   pub query: &'static str,

   // cites - citaction id to trigger "cited by"
   cite_id: Option<&'static str>,

   // as_ylo - give results from this year onwards
   from_year: Option<u16>,

   // as_yhi
   to_year: Option<u16>,

   // scisbd - 0 for relevence, 1 to include only abstracts, 2 for everything. Default = date
   sort_by: Option<u8>,

   // cluster - query all versions. Use with q and cites prohibited
   cluster_id: Option<&'static str>,

   // hl - eg: hl=en for english
   lang: Option<&'static str>,

   // lr - one or multiple languages to limit the results to
   // eg: lr=lang_fr|lang_en
   lang_limit: Option<&'static str>,

   // num - max number of results to return
   limit: Option<u32>,

   // start - result offset. Can be used with limit for pagination
   offset: Option<u32>,

   // safe - level of filtering
   // safe=active or safe=off
   adult_filtering: Option<bool>,

   // filter - whether to give similar/ommitted results
   // filter=1 for similar results and 0 for ommitted
   include_similar_results: Option<bool>,

   // as_vis - set to 1 for including citations, otherwise 0
   include_citations: Option<bool>,
}

trait Args {
    fn get_service(&self) -> Services;
    fn get_url(&self) -> Result<String, Error>;
}

trait ScrapeResult {
    fn deserialize(&self) -> String;
}

impl ScrapeResult for ScholarResult {
    fn deserialize(&self) -> String {
        String::new()
    }
}

impl Args for ScholarArgs {
    fn get_service(&self) -> Services {
        return Services::Scholar;
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

pub enum Error {
    ConnectionError,
    ParseError,
    InvalidServiceError,
    RequiredFieldError,
    NotImplementedError,
}

enum Services {
    Scholar,
}

pub fn new(
    query: &'static str,
    cite_id: Option<&'static str>,
    from_year: Option<u16>,
    to_year: Option<u16>,
    sort_by: Option<u8>,
    cluster_id: Option<&'static str>,
    lang: Option<&'static str>,
    lang_limit: Option<&'static str>,
    limit: Option<u32>,
    offset: Option<u32>,
    adult_filtering: Option<bool>,
    include_similar_results: Option<bool>,
    include_citations: Option<bool>
) -> ScholarArgs {
    ScholarArgs{
        query,
        cite_id,
        from_year,
        to_year,
        sort_by,
        cluster_id,
        lang,
        lang_limit,
        limit,
        offset,
        adult_filtering,
        include_similar_results,
        include_citations,
    }
}

fn get_base_url<'a>(service: Services) -> &'a str {
    match service {
        Services::Scholar => "https://scholar.google.com/scholar?",
    }
}

impl Client {
    fn get_document(&self, url: &str) -> Result<&str, Error> {
        return Err(Error::NotImplementedError);
    }

    fn scrape_serialize<ScrapeResult>(&self, document: &str) -> Result<ScrapeResult, Error> {
        return Err(Error::NotImplementedError);
    }

    pub fn scrape<ScrapeResult>(&self, args: &dyn Args) -> Result<ScrapeResult, Error> {
        let url: String;
        match args.get_url() {
            Ok(u) => url = u,
            Err(e) => return Err(e),
        };
        
        let doc: &str;
        match self.get_document(&url[..]) {
            Ok(page) => doc = &page[..],
            Err(e) => return Err(e),
        };

        match self.scrape_serialize::<ScrapeResult>(doc) {
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
        let sc = new(
            "abcd", None, None, None, None, None, 
            None, None, None, None, None, None, None);

        match sc.get_url() {
            Ok(url) => assert!(url.eq("https://scholar.google.com/scholar?q=abcd"), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }

    #[test]
    fn build_url_all() {
        let sc = new(
            "abcd", Some("213123123123"), Some(2018), Some(2021), Some(0), Some("3121312312"), 
            Some("en"), Some("lang_fr|lang_en"), Some(10), Some(5), Some(true), Some(true), 
            Some(true));
        match sc.get_url() {
            Ok(url) => assert!(
                url.eq("https://scholar.google.com/scholar?q=abcd&cites=213123123123&as_ylo=2018&as_yhi=2021&scisbd=0&cluster=3121312312&hl=en&lr=lang_fr|lang_en&num=10&start=5&safe=active&filter=1&as_vis=1"), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }
}
