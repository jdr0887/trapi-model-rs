use chrono::SecondsFormat;
use merge_hashmap::Merge;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

pub type BiolinkEntity = String;
pub type BiolinkPredicate = String;
pub type CURIE = String;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub enum LogLevel {
    ERROR,
    #[default]
    WARNING,
    INFO,
    DEBUG,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeType {
    LOOKUP,
    INFERRED,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResourceRoleEnum {
    PrimaryKnowledgeSource,
    AggregatorKnowledgeSource,
    SupportingDataSource,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LogEntry {
    pub timestamp: Option<String>,

    pub level: Option<LogLevel>,

    pub code: Option<String>,

    pub message: Option<String>,
}

impl LogEntry {
    pub fn new(level: Option<LogLevel>, code: Option<String>, message: Option<String>) -> LogEntry {
        LogEntry {
            timestamp: Some(chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false)),
            level,
            code,
            message,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct NodeBinding {
    #[merge(skip)]
    pub id: CURIE,

    #[merge(skip)]
    pub query_id: Option<CURIE>,

    #[merge(skip)]
    pub attributes: Option<Vec<Attribute>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Analysis {
    pub resource_id: String,

    pub score: Option<f64>,

    pub scoring_method: Option<String>,

    pub support_graphs: Option<Vec<String>>,

    pub edge_bindings: BTreeMap<String, Vec<EdgeBinding>>,

    pub attributes: Option<Vec<Attribute>>,
}

impl Analysis {
    pub fn new(resource_id: String, edge_bindings: BTreeMap<String, Vec<EdgeBinding>>) -> Analysis {
        Analysis {
            resource_id,
            score: None,
            scoring_method: None,
            support_graphs: None,
            edge_bindings,
            attributes: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct EdgeBinding {
    pub id: String,

    pub attributes: Option<Vec<Attribute>>,
}

impl EdgeBinding {
    pub fn new(id: String) -> EdgeBinding {
        EdgeBinding { id, attributes: None }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct Result {
    #[merge(skip)]
    pub node_bindings: BTreeMap<String, Vec<NodeBinding>>,

    #[merge(strategy = merge_hashmap::vec::append)]
    pub analyses: Vec<Analysis>,
}

impl Result {
    pub fn new(node_bindings: BTreeMap<String, Vec<NodeBinding>>, analyses: Vec<Analysis>) -> Result {
        Result { node_bindings, analyses }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct Attribute {
    #[merge(skip)]
    pub attribute_type_id: CURIE,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub original_attribute_name: Option<String>,

    #[merge(skip)]
    pub value: Value,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub value_type_id: Option<CURIE>,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub attribute_source: Option<String>,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub value_url: Option<String>,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub description: Option<String>,

    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub attributes: Option<Vec<Value>>,
    // pub attributes: Option<Vec<Attribute>>,
}

impl Attribute {
    pub fn new(attribute_type_id: CURIE, value: Value) -> Attribute {
        Attribute {
            attribute_type_id,
            original_attribute_name: None,
            value,
            value_type_id: None,
            attribute_source: None,
            value_url: None,
            description: None,
            attributes: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AttributeConstraint {
    pub id: CURIE,

    pub name: String,

    pub operator: String,

    pub value: String,

    pub not: Option<bool>,

    pub unit_id: Option<String>,

    pub unit_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Qualifier {
    #[schemars(regex(pattern = r"^biolink:[a-z][a-z_]*$"))]
    pub qualifier_type_id: CURIE,

    pub qualifier_value: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct QualifierConstraint {
    pub qualifier_set: Vec<Qualifier>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct QNode {
    #[schemars(regex(pattern = r"^biolink:[a-z][a-z_]*$"))]
    pub ids: Option<Vec<CURIE>>,

    #[schemars(regex(pattern = r"^biolink:[A-Z][a-zA-Z]*$"))]
    pub categories: Option<Vec<BiolinkEntity>>,

    pub is_set: Option<bool>,

    pub constraints: Option<Vec<AttributeConstraint>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct QEdge {
    #[schemars(regex(pattern = r"^biolink:[a-z][a-z_]*$"))]
    pub predicates: Option<Vec<BiolinkPredicate>>,

    pub subject: String,

    pub object: String,

    pub knowledge_type: Option<KnowledgeType>,

    pub attribute_constraints: Option<Vec<AttributeConstraint>>,

    pub qualifier_constraints: Option<Vec<QualifierConstraint>>,

    pub provided_by: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct QueryGraph {
    pub edges: BTreeMap<String, QEdge>,
    pub nodes: BTreeMap<String, QNode>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RetrievalSource {
    pub resource_id: CURIE,

    pub resource_role: ResourceRoleEnum,

    pub upstream_resource_ids: Option<Vec<CURIE>>,

    pub source_record_urls: Option<Vec<String>>,
}

impl RetrievalSource {
    pub fn new(resource_id: CURIE, resource_role: ResourceRoleEnum) -> RetrievalSource {
        RetrievalSource {
            resource_id,
            resource_role,
            upstream_resource_ids: None,
            source_record_urls: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct Node {
    #[merge(strategy = merge_hashmap::option::overwrite_none)]
    pub name: Option<String>,

    #[merge(strategy = merge_node_categories)]
    #[schemars(regex(pattern = r"^biolink:[A-Z][a-zA-Z]*$"))]
    pub categories: Option<Vec<BiolinkEntity>>,

    #[merge(strategy = merge_attributes)]
    pub attributes: Option<Vec<Attribute>>,
}

fn merge_node_categories(left: &mut Option<Vec<BiolinkEntity>>, right: Option<Vec<BiolinkEntity>>) {
    if let Some(new) = right {
        if let Some(original) = left {
            original.extend(new);
        } else {
            *left = Some(new);
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct Edge {
    #[merge(skip)]
    pub subject: CURIE,

    #[merge(skip)]
    #[schemars(regex(pattern = r"^biolink:[a-z][a-z_]*$"))]
    pub predicate: BiolinkPredicate,

    #[merge(skip)]
    pub object: CURIE,

    #[merge(strategy = merge_hashmap::vec::append)]
    pub sources: Vec<RetrievalSource>,

    #[merge(strategy = merge_attributes)]
    pub attributes: Option<Vec<Attribute>>,

    #[merge(strategy = merge_edge_qualifiers)]
    pub qualifiers: Option<Vec<Qualifier>>,
}

impl Edge {
    pub fn new(subject: CURIE, predicate: BiolinkPredicate, object: CURIE, sources: Vec<RetrievalSource>) -> Edge {
        Edge {
            subject,
            predicate,
            object,
            sources,
            attributes: None,
            qualifiers: None,
        }
    }
}

fn merge_attributes(left: &mut Option<Vec<Attribute>>, right: Option<Vec<Attribute>>) {
    if let Some(new) = right {
        if let Some(original) = left {
            original.extend(new);
            original.sort_by(
                |a, b| match (&a.attribute_type_id, &b.attribute_type_id, &a.original_attribute_name, &b.original_attribute_name) {
                    (a_ati, b_ati, Some(a_oan), Some(b_oan)) => a_ati.cmp(b_ati).then(a_oan.cmp(b_oan)),
                    (a_ati, b_ati, None, None) => a_ati.cmp(b_ati),
                    (_, _, _, _) => Ordering::Less,
                },
            );
            original.dedup();
        } else {
            *left = Some(new);
        }
    }
}

fn merge_edge_qualifiers(left: &mut Option<Vec<Qualifier>>, right: Option<Vec<Qualifier>>) {
    if let Some(new) = right {
        if let Some(original) = left {
            original.extend(new);
            original.sort_by(|a, b| a.qualifier_type_id.partial_cmp(&b.qualifier_type_id).unwrap());
            original.dedup();
        } else {
            *left = Some(new);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct KnowledgeGraph {
    #[merge(strategy = merge_hashmap::hashmap::recurse)]
    pub edges: HashMap<String, Edge>,

    #[merge(strategy = merge_hashmap::hashmap::recurse)]
    pub nodes: HashMap<String, Node>,
}

impl KnowledgeGraph {
    pub fn new(edges: HashMap<String, Edge>, nodes: HashMap<String, Node>) -> KnowledgeGraph {
        KnowledgeGraph { nodes, edges }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct Message {
    #[merge(strategy = merge_message_results)]
    pub results: Option<Vec<Result>>,

    #[merge(skip)]
    pub query_graph: Option<QueryGraph>,

    #[merge(strategy = merge_hashmap::option::recurse)]
    pub knowledge_graph: Option<KnowledgeGraph>,

    #[merge(strategy = merge_message_auxiliary_graphs)]
    pub auxiliary_graphs: Option<BTreeMap<String, AuxiliaryGraph>>,
}

fn merge_message_results(left: &mut Option<Vec<Result>>, right: Option<Vec<Result>>) {
    if let Some(new) = right {
        if let Some(original) = left {
            original.extend(new);
        } else {
            *left = Some(new);
        }
    }
}

fn merge_message_auxiliary_graphs(left: &mut Option<BTreeMap<String, AuxiliaryGraph>>, right: Option<BTreeMap<String, AuxiliaryGraph>>) {
    if let Some(new) = right {
        if let Some(original) = left {
            original.extend(new);
        } else {
            *left = Some(new);
        }
    }
}

impl Message {
    pub fn new() -> Message {
        Message {
            results: None,
            query_graph: None,
            knowledge_graph: None,
            auxiliary_graphs: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, Merge)]
pub struct AuxiliaryGraph {
    #[merge(skip)]
    pub edges: Vec<String>,

    #[merge(strategy = merge_attributes)]
    pub attributes: Option<Vec<Attribute>>,
}

impl AuxiliaryGraph {
    pub fn new(edges: Vec<String>) -> AuxiliaryGraph {
        AuxiliaryGraph { edges, attributes: None }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Workflow {
    pub id: String,

    pub parameters: Option<BTreeMap<String, Value>>,

    pub runner_parameters: Option<BTreeMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    pub workflow: Option<Vec<Workflow>>,

    pub message: Message,

    pub status: Option<String>,

    pub description: Option<String>,

    pub logs: Option<Vec<LogEntry>>,

    pub schema_version: Option<String>,

    pub biolink_version: Option<String>,
}

impl Response {
    pub fn new(message: Message) -> Response {
        Response {
            message,
            status: None,
            description: None,
            logs: None,
            workflow: None,
            schema_version: None,
            biolink_version: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(example = "example_query")]
pub struct Query {
    pub workflow: Option<Vec<Workflow>>,
    pub message: Message,
    pub log_level: Option<LogLevel>,
    pub submitter: Option<String>,
}

fn example_query() -> Query {
    let data = r#"{
      "message": {
        "query_graph": {
          "nodes": {"n1": {"ids": ["MONDO:0009061", "MONDO:0004979"]}, "n0": {"categories": ["biolink:ChemicalEntity"]}},
          "edges": {"e0": {"subject": "n0", "object": "n1", "predicates": ["biolink:treats"], "knowledge_type": "inferred"}}
        }
      }
    }"#;
    let query: Query = serde_json::from_str(data).expect("could not parse example Query data");
    query
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(example = "example_asyncquery")]
pub struct AsyncQuery {
    pub workflow: Option<Vec<Workflow>>,
    pub message: Message,
    pub callback: String,
    pub log_level: Option<LogLevel>,
    pub submitter: Option<String>,
}

fn example_asyncquery() -> AsyncQuery {
    let data = r#"{
      "message": {
        "query_graph": {
          "nodes": {"n1": {"ids": ["MONDO:0009061", "MONDO:0004979"]}, "n0": {"categories": ["biolink:ChemicalEntity"]}},
          "edges": {"e0": {"subject": "n0", "object": "n1", "predicates": ["biolink:treats"], "knowledge_type": "inferred"}}
        }
      },
      "callback": "SOME_URL"
    }"#;
    let query: AsyncQuery = serde_json::from_str(data).expect("could not parse example Query data");
    query
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AsyncQueryResponse {
    pub job_id: String,

    pub status: Option<String>,

    pub description: Option<String>,
}

impl AsyncQueryResponse {
    pub fn new(job_id: String) -> AsyncQueryResponse {
        AsyncQueryResponse {
            job_id,
            status: None,
            description: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AsyncQueryStatusResponse {
    pub status: String,

    pub description: String,

    pub logs: Vec<LogEntry>,

    pub response_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetaAttribute {
    pub attribute_type_id: CURIE,

    pub attribute_source: Option<String>,

    pub original_attribute_names: Option<Vec<String>>,

    pub constraint_use: Option<bool>,

    pub constraint_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetaQualifier {
    pub qualifier_type_id: CURIE,

    pub applicable_values: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetaNode {
    pub id_prefixes: Vec<String>,

    pub attributes: Option<Vec<MetaAttribute>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetaEdge {
    pub subject: BiolinkEntity,

    pub predicate: BiolinkPredicate,

    pub object: BiolinkEntity,

    pub knowledge_types: Option<Vec<String>>,

    pub attributes: Option<Vec<MetaAttribute>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetaKnowledgeGraph {
    pub edges: HashMap<String, MetaEdge>,
    pub nodes: HashMap<String, MetaNode>,
}

#[cfg(test)]
mod test {
    use crate::{Analysis, Attribute, EdgeBinding, LogEntry, LogLevel, Message, NodeBinding, Query, ResourceRoleEnum, Response, CURIE};
    use merge_hashmap::Merge;
    use serde::Deserializer;
    use serde_json::{Result, Value};
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn untyped_example() {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"{
            "query_graph": {
                "nodes": {
                    "n0": { "categories": ["biolink:Disease"], "ids": ["MONDO:0005737"] },
                    "n1": { "categories": ["biolink:Gene"] }
                },
                "edges": {
                    "e01": { "subject": "n0", "object": "n1" }
                }
            },
            "knowledge_graph": {
                "nodes": {
                    "MONDO:0005737": { "categories": ["biolink:Disease"], "name": "Ebola hemorrhagic fever" },
                    "HGNC:17770": { "categories": ["biolink:Gene"], "name": "RALGAPA1" },
                    "HGNC:13236": { "categories": ["biolink:Gene"], "name": "URI1" }
                },
                "edges": {
                    "x17770": { 
                        "predicate": "biolink:related_to", 
                        "subject": "MONDO:0005737", 
                        "object": "HGNC:17770",
                        "sources": [{
                            "resource_id": "infores:kp0",
                            "resource_role": "primary_knowledge_source"
                        }]
                    },
                    "x13236": { 
                        "predicate": "biolink:related_to", 
                        "subject": "MONDO:0005737", 
                        "object": "HGNC:13236",
                        "sources": [{
                            "resource_id": "infores:kp1",
                            "resource_role": "primary_knowledge_source"
                        }]
                    }
                }
            },
            "results": [
                {
                    "node_bindings": {
                        "n0": [ { "id": "MONDO:0005737" } ],
                        "n1": [ { "id": "HGNC:17770" } ]
                    },
                    "analyses": [{
                        "resource_id": "infores:kp0",
                        "edge_bindings": {
                          "e01": [{ "id": "x13236" }]
                        }
                    }]
                }
            ]
        }"#;

        let message_result: Result<Message> = serde_json::from_str(data);
        match message_result {
            Err(err) => {
                print!("{}", err);
                assert!(false);
            }
            Ok(message) => {
                let query_graph = message.query_graph;
                assert!(query_graph.is_some());
                let ids = query_graph.and_then(|a| a.nodes.get("n0").and_then(|b| b.ids.clone()));
                assert!(ids.is_some());
                println!("{:?}", ids);
            }
        }
    }

    #[test]
    fn treats_inferred() {
        let data = r#"{ 
            "message": { 
                "query_graph": { 
                    "nodes": {"n1": {"ids": ["MONDO:0009061", "MONDO:0004979"]}, "n0": {"categories": ["biolink:ChemicalEntity"]}}, 
                    "edges": {"e0": {"subject": "n0", "object": "n1", "predicates": ["biolink:treats"], "knowledge_type": "inferred", "attribute_constraints": [{ "name": "evidence_count gt 20", "id": "biolink:evidence_count", "operator": ">", "value": "20"}]}} 
                } 
            } 
        }"#;

        let potential_query: Result<Query> = serde_json::from_str(data);
        assert!(potential_query.is_ok());
    }

    #[test]
    fn key_order() {
        let expected = r#"{
    "workflow": [
        {
            "id": "lookup",
            "runner_parameters": {
                "allowlist": ["infores:aragorn"]
            }
        },
        {
            "id":"score"
        }
    ],
    "message": {
        "query_graph": {
            "edges": {
                "e0": {
                    "predicates": [
                        "biolink:correlated_with",
                        "biolink:associated_with_likelihood_of"
                    ],
                    "subject": "n0",
                    "object": "n1",
                    "provided_by": {
                        "allowlist": [
                            "infores:automat-icees-kg",
                            "infores:cohd",
                            "infores:multiomics-ehr-risk-kp"
                        ]
                    }
                },
                "e1": {
                    "subject": "n1",
                    "object": "n2",
                    "predicates": [
                        "biolink:affects_activity_of",
                        "biolink:physically_interacts_with"
                    ],
                    "provided_by": {
                        "allowlist": [
                            "infores:text-mining-provider-targeted",
                            "infores:molepro"
                        ]
                    }
                },
                "e2": {
                    "subject": "n3",
                    "object": "n2",
                    "predicates": [
                        "biolink:affects_activity_of",
                        "biolink:physically_interacts_with"
                    ]
                },
                "e3": {
                    "subject": "n2",
                    "object": "n0",
                    "predicates": [
                        "biolink:contributes_to",
                        "biolink:associated_with",
                        "biolink:gene_associated_with_condition"
                    ]
                }
            },
            "nodes": {
                "n0": {
                    "ids": [
                        "MONDO:0004979","MONDO:0016575","MONDO:0009061","MONDO:0018956","MONDO:0011705","MONDO:0008345","MONDO:0020066"
                    ],
                    "is_set": false
                },
                "n1": {
                    "categories": [
                        "biolink:SmallMolecule"
                    ],
                    "is_set": false
                },
                "n2": {
                    "categories": [
                        "biolink:Gene",
                        "biolink:Protein"
                    ],
                    "is_set": false
                },
                "n3": {
                    "categories": [
                        "biolink:Drug"
                    ],
                    "is_set": false
                }
            }
        }
    }
}"#;

        let query: Query = serde_json::from_str(expected).expect("Could not parse query");

        print!("query: {}", serde_json::to_string_pretty(&query).unwrap());

        // assert!(query);
    }

    #[test]
    #[should_panic]
    fn invalid_biolink_entity() {
        let data = r#"{ 
            "message": { 
                "query_graph": { 
                    "nodes": {"n1": {"ids": ["donkey", "frizzle chicken"]}, "n0": {"categories": ["biolink:ChemicalEntity"]}}, 
                    "edges": {"e0": {"subject": "n0", "object": "n1", "predicates": ["biolink:treats"], "knowledge_type": "inferred"}} 
                } 
            } 
        }"#;

        let potential_query: Result<Query> = serde_json::from_str(data);
        assert!(potential_query.is_err());
    }

    #[test]
    #[should_panic]
    fn invalid_biolink_predicate() {
        let data = r#"{ 
            "message": { 
                "query_graph": { 
                    "nodes": {"n1": {"ids": ["MONDO:0009061", "MONDO:0004979"]}, "n0": {"categories": ["poopy pants"]}}, 
                    "edges": {"e0": {"subject": "n0", "object": "n1", "predicates": ["biolink:treats"], "knowledge_type": "inferred"}} 
                } 
            } 
        }"#;

        let potential_query: Result<Query> = serde_json::from_str(data);
        assert!(potential_query.is_err());
    }

    #[test]
    #[ignore]
    fn scratch() {
        let data = fs::read_to_string("/tmp/asdf.pretty.json").unwrap();
        // let data = fs::read_to_string("/tmp/scratch-icees.json").unwrap();
        // let data = fs::read_to_string("/tmp/response_1683229618787.json").unwrap();
        let potential_query: Result<Query> = serde_json::from_str(data.as_str());
        match potential_query {
            Err(error) => {
                println!("{}", error);
                assert!(false);
            }
            Ok(query) => {
                // let pretty_query = serde_json::to_string_pretty(&query).unwrap();
                // fs::write("/tmp/scratch-icees.pretty.json", pretty_query).unwrap();
                assert!(true);
            }
        }
    }

    #[test]
    fn test_log_entry() {
        let log_entry = LogEntry::new(Some(LogLevel::ERROR), Some("QueryNotTraversable".to_string()), Some("message".to_string()));
        println!("{}", serde_json::to_string_pretty(&log_entry).unwrap());
        assert!(true);
    }

    #[test]
    #[ignore]
    fn test_merge() {
        let left_query_data = fs::read_to_string("mondo_0004979_output.pretty.json").unwrap();
        let left_query: Query = serde_json::from_str(&left_query_data).unwrap();
        let mut left_message = left_query.message;

        let right_query_data = fs::read_to_string("mondo_0009061_output.pretty.json").unwrap();
        let right_query: Query = serde_json::from_str(&right_query_data).unwrap();
        let right_message = right_query.message;

        let before_merge = match &left_message.results {
            Some(results) => results.len(),
            None => 0,
        };

        left_message.merge(right_message);

        let after_merge = match &left_message.results {
            Some(results) => results.len(),
            None => 0,
        };
        assert!(before_merge < after_merge);

        if let Some(kg) = left_message.knowledge_graph {
            if let Some(node) = kg.nodes.get("PUBCHEM.COMPOUND:16220172") {
                if let Some(attributes) = &node.attributes {
                    assert_eq!(attributes.len(), 2);
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    #[ignore]
    fn test_merge_three_files() {
        let first_response_content = fs::read_to_string("/tmp/cqs/68c1ade2-5a20-418d-a62a-2ca53a35f998.json").unwrap();
        let first_response: Response = serde_json::from_str(&first_response_content).unwrap();
        let mut first_response_message = first_response.message;

        match &first_response_message.knowledge_graph {
            Some(kg) => match (&kg.nodes, &kg.edges) {
                (nodes, edges) => {
                    println!("1st - kg.nodes.len(): {}, kg.edges.len(): {}", nodes.len(), edges.len());
                }
                _ => {}
            },
            _ => {}
        };

        let second_response_content = fs::read_to_string("/tmp/cqs/7fb94d9d-3f31-432c-b528-1b168aaca6e1.json").unwrap();
        let second_response: Response = serde_json::from_str(&second_response_content).unwrap();
        let mut second_response_message = second_response.message;

        match &second_response_message.knowledge_graph {
            Some(kg) => match (&kg.nodes, &kg.edges) {
                (nodes, edges) => {
                    println!("2nd - kg.nodes.len(): {}, kg.edges.len(): {}", nodes.len(), edges.len());
                }
                _ => {}
            },
            _ => {}
        };

        let third_response_content = fs::read_to_string("/tmp/cqs/8c93ec5e-1140-4ec1-9115-4fc3e51131fc.json").unwrap();
        let third_response: Response = serde_json::from_str(&third_response_content).unwrap();
        let mut third_response_message = third_response.message;

        match &third_response_message.knowledge_graph {
            Some(kg) => match (&kg.nodes, &kg.edges) {
                (nodes, edges) => {
                    println!("3rd - kg.nodes.len(): {}, kg.edges.len(): {}", nodes.len(), edges.len());
                }
                _ => {}
            },
            _ => {}
        };

        let first_result_count_pre_merge = match &third_response_message.results {
            Some(results) => results.len(),
            None => 0,
        };
        println!("pre merge - first: {}", first_result_count_pre_merge);

        let second_result_count_pre_merge = match &second_response_message.results {
            Some(results) => results.len(),
            None => 0,
        };
        println!("pre merge - second: {}", second_result_count_pre_merge);

        let third_result_count_pre_merge = match &third_response_message.results {
            Some(results) => results.len(),
            None => 0,
        };
        println!("pre merge - third: {}", third_result_count_pre_merge);

        first_response_message.merge(second_response_message);
        first_response_message.merge(third_response_message);

        match &first_response_message.knowledge_graph {
            Some(kg) => match (&kg.nodes, &kg.edges) {
                (nodes, edges) => {
                    println!("1st - post merge - kg.nodes.len(): {}, kg.edges.len(): {}", nodes.len(), edges.len());
                }
                _ => {}
            },
            _ => {}
        };

        let first_result_count_post_merge = match &first_response_message.results {
            Some(results) => results.len(),
            None => 0,
        };
        println!("post merge: {}", first_result_count_post_merge);
        // assert!(before_merge < after_merge);

        std::fs::write(std::path::Path::new("/tmp/test.json"), serde_json::to_string_pretty(&first_response_message).unwrap()).expect("failed to write output");

        // if let Some(kg) = left_message.knowledge_graph {
        //     if let Some(node) = kg.nodes.get("PUBCHEM.COMPOUND:16220172") {
        //         if let Some(attributes) = &node.attributes {
        //             assert_eq!(attributes.len(), 2);
        //         } else {
        //             assert!(false);
        //         }
        //     } else {
        //         assert!(false);
        //     }
        // } else {
        //     assert!(false);
        // }
    }
}
