use crate::Error;
use serde::{Deserialize, Serialize};

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
mod tooltip;

#[derive(Serialize)]
pub struct Metadata {
    name: String,
    #[serde(rename = "friendlyName")]
    friendly_name: String,
    description: String,
    author: String,
    version: String,
    link: String,
    requires: Vec<String>,
}

impl Metadata {
    #[inline]
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder {
            name: None,
            friendly_name: None,
            description: String::from(
                "This mod has been generated via speciesgen made by skylar779.",
            ),
            author: None,
            version: String::from("1.0.0_gen5.0.0"),
            link: String::from("https://github.com/skylar779/speciesgen"),
            requires: Vec::new(),
        }
    }
}

pub struct MetadataBuilder {
    name: Option<String>,
    friendly_name: Option<String>,
    description: String,
    author: Option<String>,
    version: String,
    link: String,
    requires: Vec<String>,
}

impl MetadataBuilder {
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
    pub fn description<P>(&mut self, description: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.description = description.into();
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
    pub fn version<P>(&mut self, version: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.version = version.into();
        self
    }

    #[inline]
    pub fn link<P>(&mut self, link: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.link = link.into();
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
    fn _finish(self) -> Option<Metadata> {
        let Self {
            name,
            friendly_name,
            description,
            author,
            version,
            link,
            requires,
        } = self;
        let name = name?;
        let friendly_name = friendly_name?;
        let author = author?;

        Some(Metadata {
            name,
            friendly_name,
            description,
            author,
            version,
            link,
            requires,
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
    pub fn new<I>(r#type: Subs, desc_chest: I) -> WeightstageSub
    where
        I: Into<String>,
    {
        WeightstageSub {
            name: r#type.as_str().to_string(),
            friendly_name: r#type.as_friendly().to_string(),
            desc_chest: desc_chest.into(),
            type_chest: r#type.to_chest(),
        }
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
