use serde::{Deserialize, Serialize};
use common::Enumerate;
use common::prelude::{tokio, Enumerate};
use common::prelude::tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use crate::Result;

pub(crate) async fn prompt(msg: impl AsRef<str>) -> common::Result<String> {
    let mut read = tokio::io::BufReader::new(tokio::io::stdin());

    tokio::io::stderr()
        .write_all(msg.as_ref().as_bytes())
        .await?;

    let mut str = String::new();
    read.read_line(&mut str).await?;

    Ok(str.trim().to_string())
}

#[derive(Enumerate)]
pub enum YesNo {
    #[alt("y", "j")]
    Yes,
    #[alt("n")]
    No
}

pub(crate) async fn confirm(msg: impl AsRef<str>) -> bool {
    match select(msg).await {
        Ok(YesNo::Yes) => true,
        _ => false
    }
}

pub(crate) async fn select<'a, T: Enumerate>(msg: impl AsRef<str>) -> Result<T> {
    return T::parse(&prompt(msg.as_ref()).await?)
        .ok_or(std::io::Error::other("Invalid option"));
}