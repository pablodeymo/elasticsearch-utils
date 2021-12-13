pub mod resp_struct;
pub mod search_struct;

use crate::resp_struct::RespStruct;
use anyhow::{anyhow, Result};
use elasticsearch::{http::transport::Transport, CountParts, Elasticsearch, SearchParts};
use serde_json::json;
use serde_json::Value;

pub async fn search_elastic(
    query: &mut serde_json::Value,
    query_count: &serde_json::Value,
    index: &str,
    url: &str,
    last_page: bool,
) -> Result<(RespStruct, Option<u64>)> {
    let transport = Transport::single_node(url)?;
    let client = Elasticsearch::new(transport);

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

    if last_page {
        let count_val = count.unwrap_or(0);
        let last_page = (count_val - 1) / 10;
        let new_from = last_page * 10;
        *query.get_mut("from").unwrap() = json!(new_from);
    }

    // make a search API call
    let res_elasticsearch = client
        .search(SearchParts::Index(&[index]))
        .body(query)
        .allow_no_indices(true)
        .send()
        .await?
        .json::<Value>()
        .await?;

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
