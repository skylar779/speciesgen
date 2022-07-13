use super::Op;
use crate::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct Patch {
    op: Op,
    path: String,
    value: String,
}

impl Patch {
    #[inline]
    pub fn builder() -> PatchBuilder {
        PatchBuilder {
            op: Op::Add,
            path: "/compatibleSpecies/-".to_string(),
            value: None,
        }
    }

    #[inline]
    pub fn set_op(mut self, op: Op) {
        self.op = op;
    }
}

pub struct PatchBuilder {
    op: Op,
    path: String,
    value: Option<String>,
}

impl PatchBuilder {
    #[inline]
    pub fn op(&mut self, op: Op) -> &mut Self {
        self.op = op;
        self
    }

    #[inline]
    pub fn path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.path = path.into();
        self
    }

    #[inline]
    pub fn value<P>(&mut self, value: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.value = Some(value.into());
        self
    }

    #[inline]
    fn _finish(self) -> Option<Patch> {
        let Self { op, path, value } = self;

        let value = value?;

        Some(Patch { op, path, value })
    }

    #[inline]
    pub fn finish(self) -> Result<Patch, Error> {
        self._finish().ok_or(Error::CreatePatch)
    }
}
