// Leptos Frontend Library
#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::domain;
    pub use crate::appstate;
}

// Re-export main modules for SSR
pub mod domain;
pub mod appstate;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // Future: mount Leptos app for hydration
    #[cfg(feature = "hydrate")]
    console_error_panic_hook::set_once();
}