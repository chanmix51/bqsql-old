use super::*;
use std::fmt::Debug;
use std::process::Command;

pub trait BqClient: Debug {
    fn query(&self, sql_filename: &str, dataset_id: &str) -> Result<String>;
}

#[derive(Debug)]
pub struct BqBinary {
    project_id: String,
}

impl BqBinary {
    pub fn new(project_id: &str) -> Self {
        Self {
            project_id: String::from(project_id),
        }
    }
}

impl BqClient for BqBinary {
    fn query(&self, sql_filename: &str, dataset_id: &str) -> Result<String> {
        // launch bq with that file
        let output = Command::new("/usr/bin/bq")
            .arg("--project_id")
            .arg(&self.project_id)
            .arg("--dataset_id")
            .arg(dataset_id)
            .arg("--location=EU")
            .arg("query")
            .arg("--quiet")
            .arg("--use_legacy_sql=false")
            .arg("--headless")
            .arg("--flagfile")
            .arg(sql_filename)
            .output()
            .unwrap();
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }
}
