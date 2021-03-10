//use chrono::{Duration, DurationRound};

use std::convert::{TryFrom, TryInto};

use chrono::{NaiveDate, NaiveDateTime};
use hdk3::{
    hash_path::path::{Component, Path},
    prelude::{ExternResult, SerializedBytes, UnsafeBytes, WasmError},
};

use crate::entries::{Index, IndexIndex, TimeIndex, WrappedPath};

impl IndexIndex {
    pub fn get_sb(self) -> ExternResult<SerializedBytes> {
        Ok(self.try_into()?)
    }
}

impl TryFrom<Path> for Index {
    type Error = WasmError;

    fn try_from(data: Path) -> ExternResult<Index> {
        let path_comps: Vec<Component> = data.into();
        let time_index = path_comps
            .last()
            .ok_or(WasmError::Zome(String::from(
                "Cannot get Index from empty path",
            )))?
            .to_owned();
        let time_index: Vec<u8> = time_index.into();
        let time_index = Index::try_from(SerializedBytes::from(UnsafeBytes::from(time_index)))?;
        Ok(time_index)
    }
}

impl TimeIndex {
    pub fn get_sb(self) -> ExternResult<SerializedBytes> {
        Ok(self.try_into()?)
    }
}

impl TryFrom<Component> for TimeIndex {
    type Error = WasmError;

    fn try_from(data: Component) -> Result<Self, Self::Error> {
        let time_index: Vec<u8> = data.into();
        Ok(TimeIndex::try_from(SerializedBytes::from(
            UnsafeBytes::from(time_index),
        ))?)
    }
}

impl TryInto<NaiveDateTime> for WrappedPath {
    type Error = WasmError;

    fn try_into(self) -> Result<NaiveDateTime, Self::Error> {
        let data = self.0;
        let path_comps: Vec<Component> = data.into();
        Ok(NaiveDate::from_ymd(
            TimeIndex::try_from(
                path_comps
                    .get(1)
                    .ok_or(WasmError::Zome(String::from(
                        "Expected at least two elements to convert to DateTime",
                    )))?
                    .to_owned(),
            )?
            .0 as i32,
            TimeIndex::try_from(
                path_comps
                    .get(2)
                    .unwrap_or(&Component::from(
                        SerializedBytes::try_from(TimeIndex(1))?.bytes().to_owned(),
                    ))
                    .to_owned(),
            )?
            .0,
            TimeIndex::try_from(
                path_comps
                    .get(3)
                    .unwrap_or(&Component::from(
                        SerializedBytes::try_from(TimeIndex(1))?.bytes().to_owned(),
                    ))
                    .to_owned(),
            )?
            .0,
        )
        .and_hms(
            TimeIndex::try_from(
                path_comps
                    .get(4)
                    .unwrap_or(&Component::from(
                        SerializedBytes::try_from(TimeIndex(1))?.bytes().to_owned(),
                    ))
                    .to_owned(),
            )?
            .0,
            TimeIndex::try_from(
                path_comps
                    .get(5)
                    .unwrap_or(&Component::from(
                        SerializedBytes::try_from(TimeIndex(1))?.bytes().to_owned(),
                    ))
                    .to_owned(),
            )?
            .0,
            TimeIndex::try_from(
                path_comps
                    .get(6)
                    .unwrap_or(&Component::from(
                        SerializedBytes::try_from(TimeIndex(1))?.bytes().to_owned(),
                    ))
                    .to_owned(),
            )?
            .0,
        ))
    }
}

impl From<u32> for TimeIndex {
    fn from(data: u32) -> Self {
        TimeIndex(data)
    }
}

impl Into<u32> for TimeIndex {
    fn into(self) -> u32 {
        self.0
    }
}
