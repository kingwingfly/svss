use agdb::{Db, DbId, QueryBuilder, QueryError, UserValue};
use bevy::prelude::*;

const DB_FILE: &str = concat!(env!("OUT_DIR"), "db_file.agdb");

#[derive(Resource)]
pub struct AppState {
    /// each db is a project, containing multiple graphs
    db: Db,
}

impl Default for AppState {
    fn default() -> Self {
        let mut db = Db::new(DB_FILE).unwrap_or_else(|_| {
            panic!("{} should be created", DB_FILE);
        });
        db.transaction_mut(|t| -> Result<(), QueryError> {
            let nodes = ["root", "config", "graph", "sub_config"];
            if t.exec(QueryBuilder::select().aliases().query())?.result != nodes.len() as i64 {
                t.exec_mut(QueryBuilder::insert().nodes().aliases(nodes).query())?;
                t.exec_mut(
                    QueryBuilder::insert()
                        .edges()
                        .from("root")
                        .to(["config", "graph"])
                        .query(),
                )?;
                t.exec_mut(
                    QueryBuilder::insert()
                        .edges()
                        .from("graph")
                        .to(["sub_config"])
                        .query(),
                )?;
                info!("Database initialized: {}", DB_FILE);
            }
            Ok(())
        })
        .expect("Db should be initialized");
        Self { db }
    }
}

#[derive(Debug, UserValue)]
struct Config {}

#[derive(Debug, UserValue)]
struct Graph {
    db_id: Option<DbId>,
    name: String,
    desc: String,
    path: String,
}

#[derive(Debug, UserValue)]
struct Node {
    db_id: Option<DbId>,
    content: String,
    x: f32,
    y: f32,
}
