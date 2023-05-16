#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::{sql::Object, kvs::Datastore
, dbs::Session
};

use crate::db::{AffectedRows, DB};

use cors::*;

mod cors;
mod db;
mod error;
mod prelude;
mod utils;
