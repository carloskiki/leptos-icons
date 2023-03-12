#[cfg(feature = "FaSolidO")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidO")]
/// *This icon requires the feature* `FaSolidO` *to be enabled*.
#[component]
pub fn O(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 96a160 160 0 1 0 0 320 160 160 0 1 0 0-320zM448 256A224 224 0 1 1 0 256a224 224 0 1 1 448 0z" /></svg>
   }
}