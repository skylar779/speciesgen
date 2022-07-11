use crate::command::Build;
use crate::consts;
use crate::error::Error;
use crate::fs;
use crate::model::{Category, Frame, FramesMultiple, Grid, Item, Metadata, Patch, Weightstage};
use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
use std::iter;
use std::path::PathBuf;

//mod consts;
//mod error;
//mod fs;
//mod model;

#[inline]
pub fn build(args: Build, mut weightstages: Vec<Weightstage>) {
    let mut metadata_builder = Metadata::builder();

    metadata_builder
        .author(args.author)
        .requires(["Big Fatties"])
        .name(format!("Big Fatties {} species", args.species))
        .friendly_name(format!(
            "Big Fatties - {} species support",
            args.friendly_species
        ));

    let metadata = match metadata_builder.finish() {
        Ok(metadata) => metadata,
        Err(error) => {
            println!("error: {error:?}");
            std::process::exit(1);
        }
    };

    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("./{}support", args.species)));

    if let Some(parent) = output.parent() {
        if !parent.is_dir() {
            println!("error: Output path {parent:?} is not a directory.");
            std::process::exit(1);
        }
    }

    if let Some(config_file) = args.config {
        let result = fs::read_json::<PathBuf, Vec<Weightstage>>(config_file);
    }

    if let Err(msg) = generate_mod(
        &args.species,
        args.friendly_species,
        output,
        metadata,
        weightstages,
    ) {
        println!("error: {msg}");
    }
}

#[inline]
fn generate_mod<'a>(
    species: &String,
    friendly_species: String,
    output: PathBuf,
    metadata: Metadata,
    weightstages: Vec<Weightstage>,
) -> Result<(), Error> {
    let mut patch_builder = Patch::builder();

    patch_builder.value(species);

    let patch = patch_builder.finish()?;
    let scripts_dir = output.join("scripts");
    let patch_file = output.join("scripts/bigfatties.config.patch");
    let metadata_file = output.join("_metadata");
    let weightstages_dir = output.join(format!("items/armors/weightstages/{species}"));

    let male_frames = FramesMultiple::builder()
        .body("chestm.png")
        .back_sleeve("bsleevem.png")
        .front_sleeve("fsleevem.png")
        .finish()?;

    let female_frames = FramesMultiple::builder()
        .body("chestf.png")
        .back_sleeve("bsleevef.png")
        .front_sleeve("fsleevef.png")
        .finish()?;

    fs::create_dir_all(&weightstages_dir)?;
    fs::create_dir(&scripts_dir)?;
    fs::write_json_pretty(&metadata_file, &metadata)?;
    fs::write_json_pretty(&patch_file, &[patch])?;

    for weightstage in weightstages.iter() {
        let weightstage_dir = weightstages_dir.join(&weightstage.name);

        let base_short_description = if weightstage.friendly_name.is_empty() {
            Cow::Borrowed(&friendly_species)
        } else {
            let short_description = format!("{} {}", weightstage.friendly_name, friendly_species);

            Cow::Owned(short_description)
        };

        fs::create_dir(&weightstage_dir)?;

        if weightstage.frames {
            let frames_f_file =
                weightstages_dir.join(format!("{}_pantsf.frames", weightstage.name));

            let frames_m_file =
                weightstages_dir.join(format!("{}_pantsm.frames", weightstage.name));

            let aliases = consts::FRAME_ALIASES.iter().copied();
            let names = consts::FRAME_GRID
                .iter()
                .copied()
                .map(|grid| grid.iter().copied());

            let grid = Grid::builder().names(names).finish();
            let mut frames_builder = Frame::builder();

            frames_builder.aliases(aliases).grid(grid);

            let frames = frames_builder.finish()?;

            fs::write_json_pretty(&frames_f_file, &frames)?;
            fs::write_json_pretty(&frames_m_file, &frames)?;
        }

        if weightstage.id {
            let (pantsf, pantsm) = if weightstage.frames {
                let pantsf = format!("{}_pantsf.png", weightstage.name);
                let pantsm = format!("{}_pantsm.png", weightstage.name);

                (Cow::Owned(pantsf), Cow::Owned(pantsm))
            } else {
                consts::STATIC_PANTS_IMAGES
            };

            let images = consts::BASE_IMAGES
                .iter()
                .chain(iter::once(&pantsf))
                .chain(iter::once(&pantsm));

            for image in images {
                let image_file = weightstage_dir.join(&**image);

                fs::create(&image_file)?;
            }

            let chest_file = weightstage_dir.join(format!("{}{species}.chest", weightstage.name));

            let mut chest_item_builder = Item::builder();

            chest_item_builder
                .item_name(format!("{}{species}chest", weightstage.name))
                .inventory_icon("icons.png:chest")
                .category(Category::Chestwear)
                .description(&weightstage.desc_chest)
                .short_description(format!(
                    "{base_short_description} {}",
                    weightstage.type_chest
                ))
                .male_frames(male_frames.clone())
                .female_frames(female_frames.clone());

            let chest_item = chest_item_builder.finish()?;

            fs::write_json_pretty(&chest_file, &chest_item)?;

            let leg_file = weightstage_dir.join(format!("{}{species}.legs", weightstage.name));
            let mut leg_item_builder = Item::builder();

            leg_item_builder
                .item_name(format!("{}{species}legs", weightstage.name))
                .inventory_icon("icons.png:pants")
                .category(Category::Legwear)
                .description(&weightstage.desc_leg)
                .short_description(format!("{base_short_description} {}", weightstage.type_leg))
                .male_frames("pantsm.png")
                .female_frames("pantsf.png");

            let leg_item = leg_item_builder.finish()?;

            fs::write_json_pretty(&leg_file, &leg_item)?;
        }

        if !weightstage.subs.is_empty() {
            for sub in weightstage.subs.iter() {
                let weightstagesub_dir = weightstage_dir.join(&sub.name);

                fs::create_dir(&weightstagesub_dir)?;

                for image in consts::BASE_IMAGES.iter() {
                    let image_file = weightstagesub_dir.join(&**image);

                    fs::create(&image_file)?;
                }

                let mut chest_item_builder = Item::builder();

                chest_item_builder
                    .item_name(format!("{}{}{species}chest", weightstage.name, sub.name))
                    .inventory_icon("icons.png:chest")
                    .category(Category::Chestwear)
                    .description(&sub.desc_chest)
                    .short_description(format!(
                        "{} {base_short_description} {}",
                        sub.friendly_name, sub.type_chest
                    ))
                    .male_frames(male_frames.clone())
                    .female_frames(female_frames.clone());

                let chest_item = chest_item_builder.finish()?;

                let chest_file = weightstagesub_dir
                    .join(format!("{}{}{species}.chest", weightstage.name, sub.name));

                fs::write_json_pretty(&chest_file, &chest_item)?;
            }
        }
    }

    Ok(())
}
