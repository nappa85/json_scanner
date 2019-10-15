#![deny(warnings)]
#![deny(missing_docs)]

//! # json_scanner
//!
//! Utility to scan json files contents

use std::path::PathBuf;
use std::ffi::OsStr;

use futures_util::stream::StreamExt;
use futures_util::stream::futures_unordered::FuturesUnordered;

use tokio::fs::{File, read_dir};
use tokio::io::AsyncReadExt;

use serde_json::value::Value;

use structopt::StructOpt;

use lazy_static::lazy_static;

use log::error;

#[derive(StructOpt, Debug)]
struct Opt {
    /// input folder
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// extension filter
    #[structopt(short, long)]
    pub extension: Option<String>,

    /// json snippet
    #[structopt(parse(from_os_str = parse_json))]
    pub json: Value,
}

fn parse_json(json: &OsStr) -> Value {
    serde_json::from_str(json.to_str().expect("Error decoding input json")).expect("Error parsing input json")
}

fn json_match_bool(source: &Value, compare: &Value) -> bool {
    match (source, compare) {
        (Value::Array(rows), _) => for row in rows {
            if json_match_bool(row, compare) {
                return true;
            }
        },
        (Value::Object(v1), Value::Object(v2)) => for (key, value) in v2 {
            if json_match_bool(&v1[key], value) {
                return true;
            }
        },
        _ => {
            return source == compare;
        },
    }
    false
}

fn json_match(source: &Value, compare: &Value) -> Result<Value, ()> {
    match (source, compare) {
        (Value::Array(ref rows), _) => for row in rows {
            if let Ok(v) = json_match(row, compare) {
                return Ok(v);
            }
        },
        (Value::Object(ref v1), Value::Object(ref v2)) => {
            for (key, value) in v2 {
                if !json_match_bool(&v1[key], value) {
                    return Err(());
                }
            }
            return Ok(Value::from(v1.clone()));
        },
        _ => if source == compare {
            return Ok(source.clone());
        },
    }
    Err(())
}

lazy_static! {
    static ref OPT: Opt = Opt::from_args();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let entries = read_dir(&OPT.input).await?;
    let futures = entries.fold(FuturesUnordered::new(), |mut futures, entry| async {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() && (OPT.extension.is_none() || path.extension().and_then(OsStr::to_str) == OPT.extension.as_ref().map(|s| s.as_str())) {
                futures.push(async move {
                    let mut file = match File::open(&path).await {
                        Ok(f) => f,
                        Err(e) => {
                            error!("File {} open error: {}", path.display(), e);
                            return None;
                        },
                    };
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents).await {
                        Ok(_) => {},
                        Err(e) => {
                            error!("File {} read error: {}", path.display(), e);
                            return None;
                        },
                    };

                    let json: Value = match serde_json::from_str(&contents) {
                        Ok(j) => j,
                        Err(e) => {
                            error!("File {} deserialize error: {}", path.display(), e);
                            return None;
                        },
                    };
                    if let Ok(js) = json_match(&json, &OPT.json) {
                        Some(format!("{} {}", path.display(), js))
                    }
                    else {
                        None
                    }
                });
            }

            futures
        }).await;
    futures.for_each(|s| async {
            if let Some(s) = s {
                println!("{}", s);
            }
        }).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::json_match;

    #[test]
    fn matches() {
        let source = json!([{"a":"b","c":{"d":"e","f":"g"}}]);
        let compare = json!({"a":"b","c":{"f":"g"}});
        assert_eq!(Ok(json!({"a":"b","c":{"d":"e","f":"g"}})), json_match(&source, &compare));

        let compare = json!({"a":"d","c":{"f":"b"}});
        assert_eq!(Err(()), json_match(&source, &compare));
    }
}
