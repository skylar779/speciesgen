use std::borrow::Cow;

pub const FRAME_ALIASES: &[(&str, &str)] = &[
    ("swimIdle.2", "swimIdle.1"),
    ("swim.5", "swimIdle.1"),
    ("swim.6", "swimIdle.2"),
    ("swim.7", "swimIdle.2"),
    ("lay.1", "idle.1"),
];

pub const FRAME_GRID: &[&[Option<&str>]] = &[
    &[
        None,
        Some("idle.1"),
        Some("idle.2"),
        Some("idle.3"),
        Some("idle.4"),
        Some("idle.5"),
        Some("sit.1"),
        None,
        Some("duck.1"),
    ],
    &[
        None,
        Some("walk.1"),
        Some("walk.2"),
        Some("walk.3"),
        Some("walk.4"),
        Some("walk.5"),
        Some("walk.6"),
        Some("walk.7"),
        Some("walk.8"),
    ],
    &[
        None,
        Some("run.1"),
        Some("run.2"),
        Some("run.3"),
        Some("run.4"),
        Some("run.5"),
        Some("run.6"),
        Some("run.7"),
        Some("run.8"),
    ],
    &[
        None,
        Some("jump.1"),
        Some("jump.2"),
        Some("jump.3"),
        Some("jump.4"),
        Some("fall.1"),
        Some("fall.2"),
        Some("fall.3"),
        Some("fall.4"),
    ],
    &[
        None,
        Some("climb.1"),
        Some("climb.2"),
        Some("climb.3"),
        Some("climb.4"),
        Some("climb.5"),
        Some("climb.6"),
        Some("climb.7"),
        Some("climb.8"),
    ],
    &[
        None,
        Some("swimIdle.1"),
        None,
        None,
        Some("swim.1"),
        Some("swim.2"),
        Some("swim.3"),
        Some("swim.4"),
        None,
    ],
];

pub const BASE_IMAGES: &[Cow<'static, str>] = &[
    Cow::Borrowed("chestf.png"),
    Cow::Borrowed("chestm.png"),
    Cow::Borrowed("bsleevef.png"),
    Cow::Borrowed("bsleevem.png"),
    Cow::Borrowed("fsleevef.png"),
    Cow::Borrowed("fsleevem.png"),
    Cow::Borrowed("icons.png"),
];

// static as in doesnt change, "not animated"
pub const STATIC_PANTS_IMAGES: (Cow<'static, str>, Cow<'static, str>) =
    (Cow::Borrowed("pantsf.png"), Cow::Borrowed("pantsm.png"));
