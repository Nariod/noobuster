use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: noobuster <https://hubertbonisseurdelabath.com> brute_list.txt")]
    CliUsage,
}
