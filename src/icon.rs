use std::path::Path;

use winit::window::Icon;

pub fn maybe_icon() -> Option<Icon> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            return None;
        } else {
            // You'll have to choose an icon size at your own discretion. On X11, the desired size varies
            // by WM, and on Windows, you still have to account for screen scaling. Here we use 32px,
            // since it seems to work well enough in most cases. Be careful about going too high, or
            // you'll be bitten by the low-quality downscaling built into the WM.
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/icon.png");
            return Some(load_icon(Path::new(path)));
        }
    }
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
