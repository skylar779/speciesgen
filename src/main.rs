use clap::Parser;
use cli::Cli;
use command::Commands;
use error::Error;
use model::{Weightstage, WeightstageSub};

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
                WeightstageSub::builder().name("busty").friendly_name("Busty").desc_chest("Someone is a bit well developed.").finish().unwrap(),
                WeightstageSub::builder().name("milky").friendly_name("XBusty").desc_chest("You can hear the sloshing with every bounce.").finish().unwrap(),
                WeightstageSub::builder().name("hyper").friendly_name("Hyper").desc_chest("Someone is a bit well developed.").finish().unwrap(),
                WeightstageSub::builder().name("saturated").friendly_name("Saturated").desc_chest("Got a nice small gut from that meal you just had.").finish().unwrap(),
                WeightstageSub::builder().name("stuffed").friendly_name("Stuffed").desc_chest("Quite a large meal you had with that big gut.").finish().unwrap(),
                WeightstageSub::builder().name("packed").friendly_name("Packed").desc_chest("You've since past the point you could try sucking in this gut.").finish().unwrap(),
                WeightstageSub::builder().name("glutted").friendly_name("Glutted").desc_chest("Either you're just that gluttonous or your stomach is quite elastic.").finish().unwrap(),
                WeightstageSub::builder().name("filled").friendly_name("Filled").desc_chest("You can start visualizing how much padding you're gonna get from all this food.").finish().unwrap(),
                WeightstageSub::builder().name("gorged").friendly_name("Gorged").desc_chest("Every pound in your stomach you can feel turning into fat right now.").finish().unwrap(),
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
            .subs([
                WeightstageSub::builder().name("busty").friendly_name("Busty").desc_chest("You might be a bit out of shape but at least you got some boob to balance it.").finish().unwrap(),
                WeightstageSub::builder().name("milky").friendly_name("XBusty").desc_chest("It's okay that you got a bit of a belly. Those sloshing milkers are hiding it.").finish().unwrap(),
                WeightstageSub::builder().name("hyper").friendly_name("Hyper").desc_chest("Got some real nice melons there.").finish().unwrap(),
                WeightstageSub::builder().name("stuffed").friendly_name("Stuffed").desc_chest("A bit out of shape and having a big meal, huh?").finish().unwrap(),
                WeightstageSub::builder().name("packed").friendly_name("Packed").desc_chest("You certainly ain't gonna lose that chub with a meal like this.").finish().unwrap(),
                WeightstageSub::builder().name("filled").friendly_name("Filled").desc_chest("No wonder you've packed on weight with meals like this.").finish().unwrap(),
                WeightstageSub::builder().name("gorged").friendly_name("Gorged").desc_chest("You won't be just thick with meals like this.").finish().unwrap(),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("chubster")
            .friendly_name("Chubby")
            .desc_chest("So wide, so chubby. Someone's been eating too many sweets.")
            .desc_leg(
                "Log sized legs, door-jamming hips, ultra soft rear. You've been snacking heavily.",
            )
            .subs([
                WeightstageSub::builder().name("busty").friendly_name("Busty").desc_chest("It's hard to tell your boobs are that big with a gut that still dwarfs them.").finish().unwrap(),
                WeightstageSub::builder().name("milky").friendly_name("XBusty").desc_chest("Those sloshing milk tanks help to obscure that chubby belly of yours.").finish().unwrap(),
                WeightstageSub::builder().name("hyper").friendly_name("Hyper").desc_chest("My, my. Your breasts are bigger than your own head.").finish().unwrap(),
                WeightstageSub::builder().name("stuffed").friendly_name("Stuffed").desc_chest("It's okay. It's hard to tell you've eaten with that much padding").finish().unwrap(),
                WeightstageSub::builder().name("filled").friendly_name("Filled").desc_chest("No wonder you've been packing on the pounds with that much food eaten.").finish().unwrap(),
                WeightstageSub::builder().name("gorged").friendly_name("Gorged").desc_chest("Is this a sign you've given up on losing weight? Or can you not stop yourself?").finish().unwrap(),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("plump")
            .friendly_name("Plump")
            .desc_chest("When your gut is resting on a table, it's pretty fat porky.")
            .desc_leg("A rear the size of a couch? You're really out of shape.")
            .subs([
                WeightstageSub::builder().name("busty").friendly_name("Busty").desc_chest("When your gut is resting on a table, it's pretty fat porky.").finish().unwrap(),
                WeightstageSub::builder().name("milky").friendly_name("XBusty").desc_chest("When your gut is resting on a table, it's pretty fat porky.").finish().unwrap(),
                WeightstageSub::builder().name("hyper").friendly_name("Hyper").desc_chest("Tables must fear a rack of that size.").finish().unwrap(),
                WeightstageSub::builder().name("stuffed").friendly_name("Stuffed").desc_chest("It's hard to even tell you've just eaten.").finish().unwrap(),
                WeightstageSub::builder().name("filled").friendly_name("Filled").desc_chest("A meal like this must just be an appetizer for you.").finish().unwrap(),
                WeightstageSub::builder().name("gorged").friendly_name("Gorged").desc_chest("The food in your stomach weighs more than a person, tubs.").finish().unwrap(),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("fatty")
            .friendly_name("Fatty")
            .desc_chest("Your gut is the size of a person, you're just a hopeless, fat glutton.")
            .desc_leg("That butt is an absolute dumptruck. Only good for sitting down every single day.")
            .subs([
                WeightstageSub::builder().name("busty").friendly_name("Busty").desc_chest("Such impressive boobs you have. If they weren't dwarfed by a big fat gut.").finish().unwrap(),
                WeightstageSub::builder().name("milky").friendly_name("XBusty").desc_chest("From a certain angle. Your chest can give the illusion your stomach isn't as big as it is.").finish().unwrap(),
                WeightstageSub::builder().name("hyper").friendly_name("Hyper").desc_chest("Those boobs are as wide as you are tall.").finish().unwrap(),
                WeightstageSub::builder().name("stuffed").friendly_name("Stuffed").desc_chest("Your gut is the size of a person, you're just a hopeless, fat glutton.").finish().unwrap(),
                WeightstageSub::builder().name("filled").friendly_name("Filled").desc_chest("Your gut is the size of a person, you're just a hopeless, fat glutton.").finish().unwrap(),
                WeightstageSub::builder().name("gorged").friendly_name("Gorged").desc_chest("Your gut is the size of a person, you're just a hopeless, fat glutton.").finish().unwrap(),
            ])
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("blob")
            .friendly_name("Blobby")
            .desc_chest("A bit of arm fat that can't be missed.")
            .desc_leg("It'd be amazing if you can fit into a house since you're now a whale.")
            .frames(true)
            .finish()
            .unwrap(),
        Weightstage::builder()
            .name("immobile")
            .friendly_name("Blobby")
            .desc_chest("A bit of arm fat that can't be missed.")
            .desc_leg("Now you can't even move fatty. Your gluttony knows no bounds.")
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
