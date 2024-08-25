use svg::Document;
use wmf_core::{MetafileHeader, META_EOF};

pub struct SVGGenerator<W> {
    document: Document,
    objects: Vec<crate::GraphicsObject>,
    output: W,
}

impl<W> SVGGenerator<W> {
    pub fn new(output: W) -> Self {
        Self {
            document: Document::new(),
            objects: Vec::with_capacity(0),
            output,
        }
    }
}

impl<W> crate::Generator for SVGGenerator<W>
where
    W: std::io::Write,
{
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn generate(&mut self) -> Result<(), crate::GenerateError> {
        self.output.write(&self.document.to_string().into_bytes()).map_err(
            |err| crate::GenerateError::FailedGenerate {
                cause: err.to_string(),
            },
        )?;

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn eof(&mut self, _: META_EOF) -> Result<(), crate::GenerateError> {
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn header(
        &mut self,
        header: MetafileHeader,
    ) -> Result<(), crate::GenerateError> {
        let (_placeable, header) = match header {
            MetafileHeader::StartsWithHeader(header) => (None, header),
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                (Some(placeable), header)
            }
        };

        self.objects = Vec::with_capacity(header.number_of_objects as usize);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn escape(
        &mut self,
        _record: wmf_core::META_ESCAPE,
    ) -> Result<(), crate::GenerateError> {
        Ok(())
    }
}
