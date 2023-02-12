use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: noobuster <https://kerkour.com> brute_list.txt")]
    CliUsage,
}
