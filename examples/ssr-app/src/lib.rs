use cfg_if::cfg_if;
pub mod icons;

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::icons::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            mount_to_body(|| {
                view! { <Icons /> }
            });
        }
    }
}
