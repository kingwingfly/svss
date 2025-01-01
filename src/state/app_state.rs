#[cfg(not(debug_assertions))]
use agdb::FileStorage;
#[cfg(debug_assertions)]
use agdb::MemoryStorage;
use agdb::{DbId, DbImpl, QueryBuilder, QueryError, UserValue};
use bevy::prelude::*;

#[derive(Resource)]
pub struct AppState {
    /// each db is a project, containing multiple graphs
    #[cfg(not(debug_assertions))]
    pub db: DbImpl<FileStorage>,
    #[cfg(debug_assertions)]
    pub db: DbImpl<MemoryStorage>,
}

impl Default for AppState {
    fn default() -> Self {
        let file = dirs::data_dir()
            .expect("Data dir should exist")
            .join("svss")
            .join("db_file.agdb");
        let mut db = DbImpl::new(&file.to_string_lossy()).unwrap_or_else(|_| {
            panic!("{:?} should be created", file);
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
                info!("Database initialized: {:?}", file);
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
struct NodeInfo {
    db_id: Option<DbId>,
    content: String,
    x: f32,
    y: f32,
}
