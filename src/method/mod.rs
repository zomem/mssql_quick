use serde::{Deserialize, Serialize};

#[macro_use]
mod mscount;

#[macro_use]
mod msdel;

#[macro_use]
mod msfind;

#[macro_use]
mod msget;

#[macro_use]
mod msset;

#[macro_use]
mod mssetmany;

#[macro_use]
mod msupdate;

#[macro_use]
mod msupdatemany;

mod method;
pub use method::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MssqlQuickSet {
    pub id: u64,
}

/// mscount 返回
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MssqlQuickCount {
    pub mssql_quick_count: u64,
}
