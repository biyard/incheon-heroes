use dioxus::prelude::*;

pub const FAVICON: Asset = asset!(
    "/public/logos/logo_symbol_color.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const LOGO: Asset = asset!(
    "/public/logos/logo_color.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const LOGO_WHITE: Asset = asset!(
    "/public/logos/logo_white.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const VIDEO: Asset = asset!(
    "/public/videos/main.mp4",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const DIAMOND: Asset = asset!(
    "/public/images/diamond.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const GOLD_RANK: Asset = asset!(
    "/public/images/gold_rank.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const SILVER_RANK: Asset = asset!(
    "/public/images/silver_rank.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);

pub const BRONZE_RANK: Asset = asset!(
    "/public/images/bronze_rank.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);
