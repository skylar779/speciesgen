use clap::Parser;
use cli::Cli;
use command::Commands;
use error::Error;
use model::{Chests, Legs, Subs, Weightstage, WeightstageSub};

mod cli;
mod command;
mod consts;
mod error;
mod fs;
mod model;

fn main() {
    let cli = Cli::parse();
    let weightstages: Vec<Weightstage> = vec![
        Weightstage::builder()
            .name("skinny")
            .subs([
                WeightstageSub::new(Subs::Busty, "Someone is a bit well developed."),
                WeightstageSub::new(Subs::Milky, "You can hear the sloshing with every bounce."),
                WeightstageSub::new(Subs::Hyper, "Someone is a bit well developed."),
                WeightstageSub::new(Subs::Saturated, "Got a nice small gut from that meal you just had."),
                WeightstageSub::new(Subs::Stuffed, "Quite a large meal you had with that big gut."),
                WeightstageSub::new(Subs::Packed, "You've since past the point you could try sucking in this gut."),
                WeightstageSub::new(Subs::Glutted, "Either you're just that gluttonous or your stomach is quite elastic."),
                WeightstageSub::new(Subs::Filled, "You can start visualizing how much padding you're gonna get from all this food."),
                WeightstageSub::new(Subs::Gorged, "Every pound in your stomach you can feel turning into fat right now."),
            ])
            .id(false)
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("thick")
            .friendly_name("Thick")
            .desc_chest("First sign of being out of shape is a soft gut that can't be sucked in.")
            .desc_leg(
                "Thighs touching one another? Seems like someone is a bit thicker than the most.",
            )
            .type_chest(Chests::Belly)
            .type_leg(Legs::Legs)
            .subs([
                WeightstageSub::new(Subs::Busty, "You might be a bit out of shape but at least you got some boob to balance it."),
                WeightstageSub::new(Subs::Milky, "It's okay that you got a bit of a belly. Those sloshing milkers are hiding it."),
                WeightstageSub::new(Subs::Hyper, "Got some real nice melons there."),
                WeightstageSub::new(Subs::Stuffed, "A bit out of shape and having a big meal, huh?"),
                WeightstageSub::new(Subs::Packed, "You certainly ain't gonna lose that chub with a meal like this."),
                WeightstageSub::new(Subs::Filled, "No wonder you've packed on weight with meals like this."),
                WeightstageSub::new(Subs::Gorged, "You won't be just thick with meals like this."),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("chubster")
            .friendly_name("Chubby")
            .desc_chest("So wide, so chubby. Someone's been eating too many sweets.")
            .desc_leg(
                "Log sized legs, door jamming hips, ultra soft rear. You've been snacking heavily.",
            )
            .type_chest(Chests::Belly)
            .type_leg(Legs::Legs)
            .subs([
                WeightstageSub::new(Subs::Busty, "It's hard to tell your boobs are that big with a gut that still dwarfs them."),
                WeightstageSub::new(Subs::Milky, "Those sloshing milk tanks help to obscure that chubby belly of yours."),
                WeightstageSub::new(Subs::Hyper, "My, my. Your breasts are bigger than your own head."),
                WeightstageSub::new(Subs::Stuffed, "It's okay. It's hard to tell you've eaten with that much padding."),
                WeightstageSub::new(Subs::Filled, "No wonder you've been packing on the pounds with that much food eaten."),
                WeightstageSub::new(Subs::Gorged, "Is this a sign you've given up on losing weight? Or can you not stop yourself?"),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("plump")
            .friendly_name("Plump")
            .desc_chest("When your gut is resting on a table, it's pretty fat porky.")
            .desc_leg("A rear the size of a couch? You're really out of shape.")
            .type_chest(Chests::Belly)
            .type_leg(Legs::Legs)
            .subs([
                WeightstageSub::new(Subs::Busty, "When your gut is resting on a table, it's pretty fat porky."),
                WeightstageSub::new(Subs::Milky, "When your gut is resting on a table, it's pretty fat porky."),
                WeightstageSub::new(Subs::Hyper, "Tables must fear a rack of that size."),
                WeightstageSub::new(Subs::Stuffed, "It's hard to even tell you've just eaten."),
                WeightstageSub::new(Subs::Filled, "A meal like this must just be an appetizer for you."),
                WeightstageSub::new(Subs::Gorged, "The food in your stomach weighs more than a person, tubs."),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("fatty")
            .friendly_name("Fatty")
            .desc_chest("Your gut is the size of a person, you're just a hopeless, fat glutton.")
            .desc_leg("That butt is an absolute dumptruck. Only good for sitting down every single day.")
            .type_chest(Chests::Belly)
            .type_leg(Legs::Legs)
            .subs([
                WeightstageSub::new(Subs::Busty, "Such impressive boobs you have. If they weren't dwarfed by a big fat gut."),
                WeightstageSub::new(Subs::Milky, "From a certain angle. Your chest can give the illusion your stomach isn't as big as it is."),
                WeightstageSub::new(Subs::Hyper, "Those boobs are as wide as you are tall."),
                WeightstageSub::new(Subs::Stuffed, "Your gut is the size of a person, you're just a hopeless, fat glutton."),
                WeightstageSub::new(Subs::Filled, "Your gut is the size of a person, you're just a hopeless, fat glutton."),
                WeightstageSub::new(Subs::Gorged, "Your gut is the size of a person, you're just a hopeless, fat glutton."),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("blob")
            .friendly_name("Blobby")
            .desc_chest("A bit of arm fat that can't be missed.")
            .desc_leg("It'd be amazing if you can fit into a house since you're now a whale.")
            .type_chest(Chests::Arms)
            .type_leg(Legs::Body)
            .frames(true)
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("immobile")
            .friendly_name("Blobby")
            .desc_chest("A bit of arm fat that can't be missed.")
            .desc_leg("Now you can't even move fatty. Your gluttony knows no bounds.")
            .type_chest(Chests::Arms)
            .type_leg(Legs::Body)
            .frames(true)
            .finish()
            .unwrap(),
    ];

    match cli.command {
        Commands::Build(args) => {
            command::build(args, weightstages);
        }
        Commands::Export(args) => {
            command::export(args, weightstages);
        }
    }
}
