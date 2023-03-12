#[cfg(feature = "FaSolidMound")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMound")]
/// *This icon requires the feature* `FaSolidMound` *to be enabled*.
#[component]
pub fn Mound(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M113.1 179.2C142.8 127.7 197.6 96 257 96s114.2 31.7 143.9 83.2L509.4 368c12.3 21.3-3.1 48-27.7 48H32.3c-24.6 0-40-26.6-27.7-48L113.1 179.2z" /></svg>
   }
}