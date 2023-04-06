//! Iroha smart contract functionality. Most of the traits mentioned
//! [`isi`] or Iroha Special Instructions are the main way of
//! interacting with the [`WorldStateView`], even [`wasm`] based
//! smart-contracts can only interact with the `world`, via
//! instructions.

pub mod isi;
pub mod wasm;

use std::collections::BTreeMap;

use iroha_data_model::{prelude::*, query::error::QueryExecutionFailure};
pub use isi::*;

use crate::wsv::WorldStateView;

/// Trait implementations should provide actions to apply changes on [`WorldStateView`].
pub trait Execute {
    /// Error type returned by execute function
    type Error: std::error::Error;

    /// Apply actions to `wsv` on behalf of `authority`.
    ///
    /// # Errors
    /// Concrete to each implementer.
    fn execute(self, authority: AccountId, wsv: &WorldStateView) -> Result<(), Self::Error>;
}

/// This trait should be implemented for all Iroha Queries.
pub trait ValidQuery: Query {
    /// Execute query on the [`WorldStateView`].
    /// Should not mutate [`WorldStateView`]!
    ///
    /// Returns Ok(QueryResult) if succeeded and Err(String) if failed.
    ///
    /// # Errors
    /// Concrete to each implementer
    fn execute(&self, wsv: &WorldStateView) -> Result<Self::Output, QueryExecutionFailure>;

    /// Executes query and maps it into value
    ///
    /// # Errors
    /// Concrete to each implementer
    fn execute_into_value(&self, wsv: &WorldStateView) -> Result<Value, QueryExecutionFailure> {
        self.execute(wsv).map(Into::into)
    }
}

/// Context of expression evaluation
#[derive(Clone)]
pub struct Context<'wsv> {
    values: BTreeMap<Name, Value>,
    wsv: &'wsv WorldStateView,
}

impl<'a> Context<'a> {
    /// Create new [`Self`]
    pub fn new(wsv: &'a WorldStateView) -> Self {
        Self {
            values: BTreeMap::new(),
            wsv,
        }
    }
}

impl iroha_data_model::evaluate::Context for Context<'_> {
    fn query(&self, query: &QueryBox) -> Result<Value, QueryExecutionFailure> {
        query.execute(self.wsv)
    }

    fn get(&self, name: &Name) -> Option<&Value> {
        self.values.get(name)
    }

    fn update(&mut self, other: impl IntoIterator<Item = (Name, Value)>) {
        self.values.extend(other)
    }
}
