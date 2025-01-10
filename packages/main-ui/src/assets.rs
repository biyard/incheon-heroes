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
