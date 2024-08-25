#[derive(Clone, Debug, thiserror::Error)]
pub enum GenerateError {
    #[error("failed to generate: {cause}")]
    FailedGenerate { cause: String },
    #[error("unexpected graphics object: {cause}")]
    UnexpectedGraphicsObject { cause: String },
    #[error("unknown: {cause}")]
    Unknown { cause: String },
}

pub trait Generator {
    /// Call after converting to write output.
    fn generate(&mut self) -> Result<(), GenerateError>;
    // functions to handle control record
    fn eof(&mut self, record: wmf_core::META_EOF) -> Result<(), GenerateError>;
    fn header(
        &mut self,
        header: wmf_core::MetafileHeader,
    ) -> Result<(), GenerateError>;
    // functions to handle escape record
    fn escape(
        &mut self,
        record: wmf_core::META_ESCAPE,
    ) -> Result<(), GenerateError>;
}
