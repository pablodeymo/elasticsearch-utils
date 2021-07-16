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
}

impl SearchStruct {
    pub fn new_from_hashmap_conditions(
        cond: HashMap<String, String>,
        fields_str: Option<&str>,
    ) -> SearchStruct {
        let mut vec: Vec<MatchQuery> = Vec::new();
        for (k, v) in cond {
            let mut cond_internal: HashMap<String, String> = HashMap::new();
            cond_internal.insert(k, v);

            vec.push(MatchQuery {
                match_cond: cond_internal,
            });
        }
        // convertir lista de campos separada por comas en un vector de strings
        let mut fields: Vec<String> = Vec::new();
        if let Some(fields_value) = fields_str {
            for field in fields_value.split(',') {
                fields.push(String::from(field));
            }
        }
        SearchStruct {
            query: BoolQuery {
                bool: MustQuery { must: vec },
            },
            fields,
            source: false,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchQuery {
    #[serde(rename(serialize = "match"))]
    pub match_cond: HashMap<String, String>,
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
        };

        println!("{:?}", serde_json::to_string(&q).unwrap());
    }
}
