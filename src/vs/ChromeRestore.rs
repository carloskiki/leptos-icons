#[cfg(feature = "VsChromeRestore")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChromeRestore")]
/// *This icon requires the feature* `VsChromeRestore` *to be enabled*.
#[component]
pub fn ChromeRestore(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3 5v9h9V5H3zm8 8H4V6h7v7z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M5 5h1V4h7v7h-1v1h2V3H5v2z" /></svg>
   }
}