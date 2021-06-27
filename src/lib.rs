pub mod search_struct;

use anyhow::Result;
use elasticsearch::{http::transport::Transport, Elasticsearch, SearchParts}; /*http::Method, SearchParts*/
use serde_json::Value;

pub async fn search_elastic(query: &serde_json::Value, index: &str, url: &str) -> Result<Value> {
    let transport = Transport::single_node(url)?;
    let client = Elasticsearch::new(transport);

    // make a search API call
    let res = client
        .search(SearchParts::Index(&[index]))
        .body(query)
        .allow_no_indices(true)
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("{:?}", res);

    Ok(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
