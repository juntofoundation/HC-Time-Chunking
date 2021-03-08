use chrono::{DateTime, Utc};
use hdk3::prelude::{ExternResult, EntryHash};

pub trait EntryTimeIndex {
    ///Time that entry type this trait is implemented on should be indexed under
    fn entry_time(&self) -> DateTime<Utc>;
    fn hash(&self) -> ExternResult<EntryHash>;
}
