use serde::{Deserialize, Serialize};
// use std::path::PathBuf;

use crate::Error;
pub use category::Category;
pub use chests::Chests;
pub use frame::{Frame, FrameBuilder, Frames, FramesMultiple, FramesMultipleBuilder};
pub use grid::{Grid, GridBuilder};
pub use item::{Item, ItemBuilder};
pub use legs::Legs;
pub use op::Op;
pub use patch::{Patch, PatchBuilder};
pub use rarity::Rarity;
pub use subs::Subs;
pub use subsfriendly::SubsFriendly;
pub use tooltip::Tooltip;

mod category;
mod chests;
mod frame;
mod grid;
mod item;
mod legs;
mod op;
mod patch;
mod rarity;
mod subs;
mod subsfriendly;
mod tooltip;

#[derive(Serialize)]
pub struct Metadata {
    version: String,
    author: String,
    requires: Vec<String>,
    description: String,
    name: String,
    #[serde(rename = "friendlyName")]
    friendly_name: String,
}

impl Metadata {
    #[inline]
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder {
            version: String::from("1.0.0_gen5.0.0"),
            author: None,
            requires: Vec::new(),
            description: String::from(
                "This mod has been generated with the Species Support Generator made by skylar779.",
            ),
            name: None,
            friendly_name: None,
        }
    }
}

pub struct MetadataBuilder {
    version: String,
    author: Option<String>,
    requires: Vec<String>,
    description: String,
    name: Option<String>,
    friendly_name: Option<String>,
}

impl MetadataBuilder {
    #[inline]
    pub fn version<P>(&mut self, version: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.version = version.into();
        self
    }

    #[inline]
    pub fn author<P>(&mut self, author: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.author = Some(author.into());
        self
    }

    #[inline]
    pub fn requires<I, S>(&mut self, requires: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.requires.extend(requires.into_iter().map(Into::into));
        self
    }

    #[inline]
    pub fn description<P>(&mut self, description: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.description = description.into();
        self
    }

    #[inline]
    pub fn name<P>(&mut self, name: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    #[inline]
    pub fn friendly_name<P>(&mut self, friendly_name: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.friendly_name = Some(friendly_name.into());
        self
    }

    #[inline]
    fn _finish(self) -> Option<Metadata> {
        let Self {
            version,
            author,
            requires,
            description,
            name,
            friendly_name,
        } = self;
        let author = author?;
        let name = name?;
        let friendly_name = friendly_name?;

        Some(Metadata {
            version,
            author,
            requires,
            description,
            name,
            friendly_name,
        })
    }

    #[inline]
    pub fn finish(self) -> Result<Metadata, Error> {
        self._finish().ok_or(Error::CreateMetadata)
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeightstageSub {
    pub name: String,
    pub friendly_name: String,
    pub desc_chest: String,
    pub type_chest: Chests,
}

impl WeightstageSub {
    #[inline]
    pub fn builder() -> WeightstageSubBuilder {
        WeightstageSubBuilder {
            name: None,
            friendly_name: None,
            desc_chest: None,
            type_chest: Chests::Chest,
        }
    }
}

pub struct WeightstageSubBuilder {
    name: Option<String>,
    friendly_name: Option<String>,
    desc_chest: Option<String>,
    type_chest: Chests,
}

impl WeightstageSubBuilder {
    #[inline]
    pub fn name<P>(&mut self, name: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    #[inline]
    pub fn friendly_name<P>(&mut self, friendly_name: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.friendly_name = Some(friendly_name.into());
        self
    }

    #[inline]
    pub fn desc_chest<P>(&mut self, desc_chest: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.desc_chest = Some(desc_chest.into());
        self
    }

    #[inline]
    pub fn type_chest(&mut self, type_chest: Chests) -> &mut Self {
        self.type_chest = type_chest;
        self
    }

    #[inline]
    pub fn finish(self) -> Option<WeightstageSub> {
        let Self {
            name,
            friendly_name,
            desc_chest,
            type_chest,
        } = self;
        let name = name?;
        let friendly_name = friendly_name?;
        let desc_chest = desc_chest?;

        Some(WeightstageSub {
            name,
            friendly_name,
            desc_chest,
            type_chest,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Weightstage {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub friendly_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desc_chest: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desc_leg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_chest: Option<Chests>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_leg: Option<Legs>,
    pub frames: bool,
    pub id: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subs: Vec<WeightstageSub>,
}

impl Weightstage {
    #[inline]
    pub fn builder() -> WeightstageBuilder {
        WeightstageBuilder {
            name: None,
            friendly_name: None,
            desc_chest: None,
            desc_leg: None,
            type_chest: None,
            type_leg: None,
            frames: false,
            id: true,
            subs: Vec::new(),
        }
    }
}

pub struct WeightstageBuilder {
    name: Option<String>,
    friendly_name: Option<String>,
    desc_chest: Option<String>,
    desc_leg: Option<String>,
    type_chest: Option<Chests>,
    type_leg: Option<Legs>,
    frames: bool,
    id: bool,
    subs: Vec<WeightstageSub>,
}

impl WeightstageBuilder {
    #[inline]
    pub fn name<P>(mut self, name: P) -> Self
    where
        P: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    #[inline]
    pub fn friendly_name<P>(mut self, friendly_name: P) -> Self
    where
        P: Into<String>,
    {
        self.friendly_name = Some(friendly_name.into());
        self
    }

    #[inline]
    pub fn desc_chest<P>(mut self, desc_chest: P) -> Self
    where
        P: Into<String>,
    {
        self.desc_chest = Some(desc_chest.into());
        self
    }

    #[inline]
    pub fn desc_leg<P>(mut self, desc_leg: P) -> Self
    where
        P: Into<String>,
    {
        self.desc_leg = Some(desc_leg.into());
        self
    }

    #[inline]
    pub fn type_chest(mut self, type_chest: Chests) -> Self {
        self.type_chest = Some(type_chest);
        self
    }

    #[inline]
    pub fn type_leg(mut self, type_leg: Legs) -> Self {
        self.type_leg = Some(type_leg);
        self
    }

    #[inline]
    pub fn frames(mut self, frames: bool) -> Self {
        self.frames = frames;
        self
    }

    #[inline]
    pub fn id(mut self, id: bool) -> Self {
        self.id = id;
        self
    }

    #[inline]
    pub fn subs<I, S>(mut self, subs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<WeightstageSub>,
    {
        self.subs.extend(subs.into_iter().map(Into::into));
        self
    }

    #[inline]
    pub fn finish(self) -> Option<Weightstage> {
        let Self {
            name,
            friendly_name,
            desc_chest,
            desc_leg,
            type_chest,
            type_leg,
            frames,
            id,
            subs,
        } = self;
        let name = name?;
        let friendly_name = friendly_name.unwrap_or_default();
        let desc_chest = desc_chest.unwrap_or_default();
        let desc_leg = desc_leg.unwrap_or_default();

        Some(Weightstage {
            name,
            friendly_name,
            desc_chest,
            desc_leg,
            type_chest,
            type_leg,
            frames,
            id,
            subs,
        })
    }
}

#[macro_export]
macro_rules! sub {
    ( $n:expr, $f:expr, $d:expr, $t:expr ) => {{
        let mut weightstage_builder = WeightstageSub::builder();

        weightstage_builder
            .name($n)
            .friendly_name($f)
            .desc_chest($d)
            .type_chest($t);

        weightstage_builder.finish().unwrap()
    }};
}
