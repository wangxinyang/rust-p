use pgx::*;
use serde::{Deserialize, Serialize};
use uuid7::Uuid;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    PostgresType,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
    Serialize,
    Deserialize,
)]
#[inoutfuncs]
pub struct SortedId {
    inner: Uuid,
}

impl Default for SortedId {
    fn default() -> Self {
        Self {
            inner: uuid7::uuid7(),
        }
    }
}

impl InOutFuncs for SortedId {
    fn input(input: &pgx::cstr_core::CStr) -> Self
    where
        Self: Sized,
    {
        let id = input
            .to_str()
            .expect("not a valid input")
            .parse()
            .expect("invalid uuid");
        Self { inner: id }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.inner.to_string())
    }
}

#[pg_extern]
fn get_sorted_id() -> SortedId {
    SortedId::default()
}
