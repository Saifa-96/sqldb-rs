use planner::Planner;

use crate::error::Result;
use super::{
    engine::Transaction,
    executor::{Executor, ResultSet},
    parser::ast::{self, Expression},
    schema::Table,
};

mod planner;

pub enum Node {
    CreateTable {
        schema: Table,
    },

    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },

    Scan {
        table_name: String,
    },
}

pub struct Plan(pub Node);

impl Plan {
    pub fn build(stmt: ast::Statement) -> Self {
        Planner::new().build(stmt)
    }

    pub fn execute<T: Transaction>(self, txn: &mut T) -> Result<ResultSet> {
        <dyn Executor<T>>::build(self.0).execute(txn)
    }
}
