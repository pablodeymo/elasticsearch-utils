pub mod resp_struct;
pub mod search_struct;

use crate::resp_struct::RespStruct;
use anyhow::{anyhow, Result};
use elasticsearch::{http::transport::Transport, CountParts, Elasticsearch, SearchParts};
use serde_json::Value;

pub async fn search_elastic(
    query: &serde_json::Value,
    query_count: &serde_json::Value,
    index: &str,
    url: &str,
) -> Result<(RespStruct, Option<u64>)> {
    let transport = Transport::single_node(url)?;
    let client = Elasticsearch::new(transport);

    // make a search API call
    let res_elasticsearch = client
        .search(SearchParts::Index(&[index]))
        .body(query)
        .allow_no_indices(true)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let res_count_elasticsearch = client
        .count(CountParts::Index(&[index]))
        .body(query_count)
        .allow_no_indices(true)
        .send()
        .await?
        .json::<Value>()
        .await?;

    //let ret_string = res_count_elasticsearch.to_string();
    let count = match res_count_elasticsearch.get("count") {
        None => None,
        Some(v) => match v {
            Value::Number(c) => c.as_u64(),
            _ => Some(0_u64),
        },
    };
    let ret: RespStruct = serde_json::from_value(res_elasticsearch)
        .map_err(|_e| anyhow!("Error parsing Elasticsearch response"))?;
    Ok((ret, count))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
