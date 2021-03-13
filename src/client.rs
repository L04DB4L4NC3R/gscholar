struct Client {}

pub struct ScholarArgs {
   // q - required
   query: &str,

   // cites - citaction id to trigger "cited by"
   cite_id: &str,

   // as_ylo - give results from this year onwards
   from_year: u16,

   // as_yhi
   to_year: u16,

   // scisbd - 0 for relevence, 1 to include only abstracts, 2 for everything. Default = date
   sort_by: u8,

   // cluster - query all versions. Use with q and cites prohibited
   cluster_id: &str,

   // hl - eg: hl=en for english
   lang: &str,

   // lr - one or multiple languages to limit the results to
   // eg: lr=lang_fr|lang_en
   lang_limit: &str,

   // num - max number of results to return
   limit: u32,

   // start - result offset. Can be used with limit for pagination
   offset: u32,

   // safe - level of filtering
   // safe=active or safe=off
   adult_filtering: bool,

   // filter - whether to give similar/ommitted results
   // filter=1 for similar results and 0 for ommitted
   include_similar_results: bool,

   // as_vis - set to 1 for including citations, otherwise 0
   include_citations: bool,
}

trait Args {
    fn get_service() -> Services;
    fn get_url(&self) -> Result<&str, Error>;
}

impl Args for ScholarArgs {
    fn get_service() -> Services {
        return Services::Scholar;
    }

    fn get_url(&self) -> Result<&str, Error> {
       let mut url = String::new();
       url.push_str(get_base_url(self::get_service()));
    }
}

pub enum Error {
    Nil,
    ConnectionError,
    ParseError,
    InvalidServiceError,
}

enum Services {
    Scholar,
}


fn get_base_url(service: Services) -> &str {
    return match service {
        Services::Scholar => "https://scholar.google.com/scholar?",
    }
}

impl Client {
    fn get_document(url: &str) -> Result<&str, Error> {

    }

    fn scrape_serialize(document: &str) -> Result<T, Error> {

    }

    pub fn scrape(args: Args) -> Result<T, Error> {

    }
}
