use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::W, utils::macros::map};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{thing, Array, Object, Value}, 
    Response, kvs::Datastore, dbs::Session
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Task> for Value {
    fn from(val: Task) -> Self {
        match val.id {
            Some(v) => map![
                "id".into() => v.into(),
                "title".into() => val.title.into(),
                "completed".into() => val.completed.into(),
            ]
            .into(),
            None => map![
                "title".into() => val.title.into(),
                "completed".into() => val.completed.into(),
            ]
            .into(),
        }
    }
}

impl Creatable for Task {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub affected_rows: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

pub trait Creatable: Into<Value> {}

#[derive(Clone)]
pub struct DB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl DB {
    pub async fn execute(&self, query: &str, vars: Option<BTreeMap<String, Value>>) -> Result<Vec<surrealdb::dbs::Response>, crate::error::Error> {
        let res = match self.ds.execute(query, &self.sesh, vars, false).await {
            Ok(res) => res,
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };
        Ok(res)
    }

    pub async fn add_task(&self, title: String) -> Result<Object, crate::error::Error> {
        let sql = "CREATE tasks SET title = $title, completed = false, created_at = time.now()";
        let vars: BTreeMap<String, Value> = map!["title".into() => Value::Strand(title.into())];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("no response");
        let response = match first_res.result {
            Ok(v) => v.first(),
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };
        W(response).try_into()
    }

    pub async fn get_task(&self, id: String) -> Result<Object, crate::error::Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("{}", id);
        let thing = match thing(&tid) {
            Ok(v) => v,
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };
        let vars: BTreeMap<String, Value> = map!["th".into() => thing.into()];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("no response");
        let response = match first_res.result {
            Ok(v) => v.first(),
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };

        W(response).try_into()
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Object>, crate::error::Error> {
        let sql = "SELECT * FROM tasks ORDER BY created_at ASC;";

        let ress = self.execute(sql, None).await?;

        let first_res = ress.into_iter().next().expect("no response");

        let response = match first_res.result {
            Ok(v) => v.first(),
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };

        let array: Array = W(response).try_into()?;

        array.into_iter().map(|v| W(v).try_into()).collect() 
    }

    pub async fn toggle_task(&self, id: String) -> Result<AffectedRows, crate::error::Error> {
        let sql = "UPDATE $th SET completed = function() { return !this.completed; }";
        let tid = format!("{}", id);
        let thing = match thing(&tid) {
            Ok(v) => v,
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };
        let vars: BTreeMap<String, Value> = map!["th".into() => thing.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { affected_rows: 1 })
    }

    pub async fn delete_task(&self, id: String) -> Result<AffectedRows, crate::error::Error> {
        let sql = "DELETE FROM $th";
        let tid = format!("{}", id);
        let thing = match thing(&tid) {
            Ok(v) => v,
            Err(e) => {
                return Err(crate::error::Error::Surreal(surrealdb::Error::Db(e)));
            }
        };
        let vars: BTreeMap<String, Value> = map!["th".into() => thing.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { affected_rows: 1 })
    }
}



