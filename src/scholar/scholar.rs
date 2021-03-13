struct Client {}

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
           url.push_str("&scisbd=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.cluster_id {
           url.push_str("&cluster=");
           url.push_str(i);
       }
       if let Some(i) = self.lang {
           url.push_str("&hl=");
           url.push_str(i);
       }
       if let Some(i) = self.lang_limit {
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
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.include_similar_results {
           url.push_str("&filter=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.include_citations {
           url.push_str("&as_vis=");
           url.push_str(&i.to_string()[..]);
       }
       if let Some(i) = self.adult_filtering {
           url.push_str("&adult_filtering=");
           url.push_str(&i.to_string()[..]);
       }

       return Ok(url);
    }
}

pub enum Error {
    Nil,
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
    fn get_document(url: &str) -> Result<&str, Error> {
        return Err(Error::NotImplementedError);
    }

    fn scrape_serialize<T>(document: &str) -> Result<T, Error> {
        return Err(Error::NotImplementedError);
    }

    pub fn scrape<T>(args: &dyn Args) -> Result<(), Error> {
        let url : String;
        match args.get_url() {
            Ok(u) => url = u,
            Err(e) => return Err(e),
        };
        
        print!("{}", url);
        return Ok(());
    }
}
