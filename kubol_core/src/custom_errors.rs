use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("This is testing error of the kubol engine")]
    TestError,
}