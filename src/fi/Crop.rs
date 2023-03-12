#[cfg(feature = "FiCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCrop")]
/// *This icon requires the feature* `FiCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6.13 1L6 16a2 2 0 0 0 2 2h15" /><path d="M1 6.13L16 6a2 2 0 0 1 2 2v15" /></svg>
   }
}