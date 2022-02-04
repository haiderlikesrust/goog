use serde::Deserialize;
use serde::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub kind: String,
    pub url: Url,
    pub queries: Queries,
    pub context: Context,
    pub search_information: SearchInformation,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    #[serde(rename = "type")]
    pub type_field: String,
    pub template: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queries {
    pub request: Vec<Request>,
    pub next_page: Vec<NextPage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub title: String,
    pub total_results: String,
    pub search_terms: String,
    pub count: i64,
    pub start_index: i64,
    pub input_encoding: String,
    pub output_encoding: String,
    pub safe: String,
    pub cx: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextPage {
    pub title: String,
    pub total_results: String,
    pub search_terms: String,
    pub count: i64,
    pub start_index: i64,
    pub input_encoding: String,
    pub output_encoding: String,
    pub safe: String,
    pub cx: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub title: String,
    pub facets: Vec<Vec<Facet>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    pub anchor: String,
    pub label: String,
    #[serde(rename = "label_with_op")]
    pub label_with_op: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInformation {
    pub search_time: f64,
    pub formatted_search_time: String,
    pub total_results: String,
    pub formatted_total_results: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub kind: String,
    pub title: String,
    pub html_title: String,
    pub link: String,
    pub display_link: String,
    pub snippet: String,
    pub html_snippet: String,
    pub cache_id: String,
    pub formatted_url: String,
    pub html_formatted_url: String,
    pub pagemap: Option<Pagemap>,
    pub mime: Option<String>,
    pub file_format: Option<String>,
    #[serde(default)]
    pub labels: Vec<Label>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagemap {
    pub metatags: Vec<Metatag>,
    #[serde(rename = "cse_thumbnail")]
    #[serde(default)]
    pub cse_thumbnail: Vec<CseThumbnail>,
    #[serde(rename = "cse_image")]
    #[serde(default)]
    pub cse_image: Vec<CseImage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metatag {
    pub viewport: Option<String>,
    #[serde(rename = "og:image")]
    pub og_image: Option<String>,
    pub author: Option<String>,
    #[serde(rename = "og:title")]
    pub og_title: Option<String>,
    pub moddate: Option<String>,
    pub creationdate: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub title: Option<String>,
    pub fullbanner: Option<String>,
    pub subject: Option<String>,
    pub pdfversion: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CseThumbnail {
    pub src: String,
    pub width: String,
    pub height: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CseImage {
    pub src: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: String,
    pub display_name: String,
    #[serde(rename = "label_with_op")]
    pub label_with_op: String,
}
