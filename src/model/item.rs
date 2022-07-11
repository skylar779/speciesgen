use super::{Category, Frames, Rarity, Tooltip};
use crate::Error;
use serde::Serialize;
use std::collections::HashMap;
// use std::vec::Vec;
// use std::path::PathBuf;

#[derive(Serialize)]
pub struct Item {
    #[serde(rename = "itemName")]
    item_name: String,
    price: u32,
    #[serde(rename = "inventoryIcon")]
    inventory_icon: String,
    #[serde(rename = "maxStack")]
    max_stack: u16,
    rarity: Rarity,
    category: Category,
    description: String,
    #[serde(rename = "shortdescription")]
    short_description: String,
    #[serde(rename = "tooltipKind")]
    tooltip_kind: Tooltip,
    #[serde(rename = "maleFrames")]
    male_frames: Frames,
    #[serde(rename = "femaleFrames")]
    female_frames: Frames,
    #[serde(rename = "colorOptions", skip_serializing_if = "Vec::is_empty")]
    color_options: Vec<HashMap<String, String>>,
}

impl Item {
    #[inline]
    pub fn builder() -> ItemBuilder {
        ItemBuilder {
            item_name: None,
            price: 0,
            inventory_icon: None,
            max_stack: 1,
            rarity: Rarity::Legendary,
            category: None,
            description: None,
            short_description: None,
            tooltip_kind: Tooltip::Armor,
            male_frames: None,
            female_frames: None,
            color_options: Vec::new(),
        }
    }
}

pub struct ItemBuilder {
    item_name: Option<String>,
    price: u32,
    inventory_icon: Option<String>,
    max_stack: u16,
    rarity: Rarity,
    category: Option<Category>,
    description: Option<String>,
    short_description: Option<String>,
    tooltip_kind: Tooltip,
    male_frames: Option<Frames>,
    female_frames: Option<Frames>,
    color_options: Vec<HashMap<String, String>>,
}

impl ItemBuilder {
    #[inline]
    pub fn item_name<P>(&mut self, item_name: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.item_name = Some(item_name.into());
        self
    }

    #[inline]
    pub fn price(&mut self, price: u32) -> &mut Self {
        self.price = price;
        self
    }

    #[inline]
    pub fn inventory_icon<P>(&mut self, inventory_icon: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.inventory_icon = Some(inventory_icon.into());
        self
    }

    #[inline]
    pub fn max_stack(&mut self, max_stack: u16) -> &mut Self {
        self.max_stack = max_stack;
        self
    }

    #[inline]
    pub fn rarity(&mut self, rarity: Rarity) -> &mut Self {
        self.rarity = rarity;
        self
    }

    #[inline]
    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = Some(category);
        self
    }

    #[inline]
    pub fn description<P>(&mut self, description: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    #[inline]
    pub fn short_description<P>(&mut self, short_description: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.short_description = Some(short_description.into());
        self
    }

    #[inline]
    pub fn tooltip_kind(&mut self, tooltip_kind: Tooltip) -> &mut Self {
        self.tooltip_kind = tooltip_kind;
        self
    }

    #[inline]
    pub fn male_frames<F>(&mut self, frames: F) -> &mut Self
    where
        F: Into<Frames>,
    {
        self.male_frames = Some(frames.into());
        self
    }

    #[inline]
    pub fn female_frames<F>(&mut self, frames: F) -> &mut Self
    where
        F: Into<Frames>,
    {
        self.female_frames = Some(frames.into());
        self
    }

    #[inline]
    pub fn color_options<I, S>(&mut self, options: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<HashMap<String, String>>,
    {
        let iter = options.into_iter().map(Into::into);

        self.color_options.extend(iter);
        self
    }

    #[inline]
    fn _finish(self) -> Option<Item> {
        let Self {
            item_name,
            price,
            inventory_icon,
            max_stack,
            rarity,
            category,
            description,
            short_description,
            tooltip_kind,
            male_frames,
            female_frames,
            color_options,
        } = self;

        let item_name = item_name?;
        let inventory_icon = inventory_icon?;
        let category = category?;
        let description = description?;
        let short_description = short_description?;
        let male_frames = male_frames?;
        let female_frames = female_frames?;
        // let color_options = (!color_options.is_empty()).then(|| color_options);

        Some(Item {
            item_name,
            price,
            inventory_icon,
            max_stack,
            rarity,
            category,
            description,
            short_description,
            tooltip_kind,
            male_frames,
            female_frames,
            color_options,
        })
    }

    #[inline]
    pub fn finish(self) -> Result<Item, Error> {
        self._finish().ok_or(Error::CreateItem)
    }
}
