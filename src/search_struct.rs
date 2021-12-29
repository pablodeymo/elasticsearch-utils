use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Modeliza la query:
/// {
///     "query": {
///         "bool": {
///             "must": [
///                 { "match": { "coleccion": "ConsejoEstado" } },
///                 { "match": { "anio_providencia": "2019" } }
///             ]
///         }
///     }
/// }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchStruct {
    pub query: BoolQuery,
    pub fields: Vec<String>,
    #[serde(rename = "_source")]
    pub source: bool,
    pub sort: Option<HashMap<String, SortType>>,
    pub from: u64,
    pub size: u64,
}

impl SearchStruct {
    pub fn new_from_hashmap_conditions(
        cond: &HashMap<String, String>,
        fields_str: Option<&str>,
        separator: &str,
        from: u64,
        size: u64,
        sort_input: Option<&str>,
    ) -> SearchStruct {
        let mut vec: Vec<MatchQuery> = Vec::new();
        for (k, v) in cond {
            let mut cond_internal: HashMap<String, String> = HashMap::new();
            cond_internal.insert(k.to_string(), v.to_string());

            vec.push(MatchQuery {
                match_cond: cond_internal,
            });
        }
        // convertir lista de campos separada por comas en un vector de strings
        let mut fields: Vec<String> = Vec::new();
        if let Some(fields_value) = fields_str {
            for field in fields_value.split(separator) {
                fields.push(String::from(field));
            }
        }

        let sort = match sort_input {
            None => None,
            Some(s) => {
                let mut sort_hash: HashMap<String, SortType> = HashMap::new();
                sort_hash.insert(
                    s.to_string(),
                    SortType {
                        order: "asc".to_string(),
                    },
                );
                Some(sort_hash)
            }
        };

        SearchStruct {
            query: BoolQuery {
                bool: MustQuery { must: vec },
            },
            fields,
            source: false,
            sort,
            from,
            size,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoolQuery {
    pub bool: MustQuery,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MustQuery {
    pub must: Vec<MatchQuery>,
}

/// Struct to get the count of records
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryStruct {
    pub query: BoolQuery,
}

impl QueryStruct {
    pub fn new_from_hashmap_conditions(cond: &HashMap<String, String>) -> QueryStruct {
        let mut vec: Vec<MatchQuery> = Vec::new();
        for (k, v) in cond {
            let mut cond_internal: HashMap<String, String> = HashMap::new();
            cond_internal.insert(k.to_string(), v.to_string());

            vec.push(MatchQuery {
                match_cond: cond_internal,
            });
        }
        QueryStruct {
            query: BoolQuery {
                bool: MustQuery { must: vec },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchQuery {
    #[serde(rename(serialize = "match"))]
    pub match_cond: HashMap<String, String>,
}

// {"order" : "asc"}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortType {
    pub order: String,
}

#[cfg(test)]
mod tests {
    use super::{BoolQuery, MatchQuery, MustQuery, SearchStruct};
    use std::collections::HashMap;

    #[test]
    fn test_serilize() {
        let mut cond1: HashMap<String, String> = HashMap::new();
        cond1.insert("coleccion".to_string(), "ConsejoEstado".to_string());
        let mut cond2: HashMap<String, String> = HashMap::new();
        cond2.insert("anio_providencia".to_string(), "2019".to_string());

        let q = SearchStruct {
            query: BoolQuery {
                bool: MustQuery {
                    must: vec![
                        MatchQuery { match_cond: cond1 },
                        MatchQuery { match_cond: cond2 },
                    ],
                },
            },
            fields: Vec::new(),
            source: false,
            from: 0,
            size: 10,
        };

        println!("{:?}", serde_json::to_string(&q).unwrap());
    }
}
