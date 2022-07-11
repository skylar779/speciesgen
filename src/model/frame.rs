use super::Grid;
use crate::Error;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Frame {
    aliases: HashMap<String, String>,
    #[serde(rename = "frameGrid")]
    grid: Grid,
}

impl Frame {
    #[inline]
    pub fn builder() -> FrameBuilder {
        FrameBuilder {
            aliases: HashMap::new(),
            grid: None,
        }
    }
}

pub struct FrameBuilder {
    aliases: HashMap<String, String>,
    grid: Option<Grid>,
}

impl FrameBuilder {
    #[inline]
    pub fn aliases<I, K, V>(&mut self, aliases: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        #[inline]
        fn map_pair<K, V>((k, v): (K, V)) -> (String, String)
        where
            K: Into<String>,
            V: Into<String>,
        {
            (k.into(), v.into())
        }

        let iter = aliases.into_iter().map(map_pair);

        self.aliases.extend(iter);
        self
    }

    #[inline]
    pub fn grid(&mut self, grid: Grid) -> &mut Self {
        self.grid = Some(grid);
        self
    }

    #[inline]
    fn _finish(self) -> Option<Frame> {
        let Self { aliases, grid } = self;

        let grid = grid?;

        Some(Frame { aliases, grid })
    }

    #[inline]
    pub fn finish(self) -> Result<Frame, Error> {
        self._finish().ok_or(Error::CreateFrame)
    }
}

#[derive(Serialize, Clone)]
pub struct FramesMultiple {
    body: PathBuf,
    back_sleeve: PathBuf,
    front_sleeve: PathBuf,
}

impl FramesMultiple {
    #[inline]
    pub fn builder() -> FramesMultipleBuilder {
        FramesMultipleBuilder {
            body: None,
            back_sleeve: None,
            front_sleeve: None,
        }
    }
}

pub struct FramesMultipleBuilder {
    body: Option<PathBuf>,
    back_sleeve: Option<PathBuf>,
    front_sleeve: Option<PathBuf>,
}

impl FramesMultipleBuilder {
    #[inline]
    pub fn body<P>(mut self, body: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.body = Some(body.into());
        self
    }

    #[inline]
    pub fn back_sleeve<P>(mut self, back_sleeve: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.back_sleeve = Some(back_sleeve.into());
        self
    }

    #[inline]
    pub fn front_sleeve<P>(mut self, front_sleeve: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.front_sleeve = Some(front_sleeve.into());
        self
    }

    #[inline]
    fn _finish(self) -> Option<FramesMultiple> {
        let Self {
            body,
            back_sleeve,
            front_sleeve,
        } = self;

        let body = body?;
        let back_sleeve = back_sleeve?;
        let front_sleeve = front_sleeve?;

        Some(FramesMultiple {
            body,
            back_sleeve,
            front_sleeve,
        })
    }

    #[inline]
    pub fn finish(self) -> Result<FramesMultiple, Error> {
        self._finish().ok_or(Error::CreateFramesMultiple)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Frames {
    Single(PathBuf),
    Multiple(FramesMultiple),
}

impl<P> From<P> for Frames
where
    P: Into<PathBuf>,
{
    #[inline]
    fn from(path: P) -> Self {
        Self::Single(path.into())
    }
}

impl From<FramesMultiple> for Frames {
    #[inline]
    fn from(frames: FramesMultiple) -> Self {
        Self::Multiple(frames)
    }
}
