#[cfg(feature = "FiLayers")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiLayers")]
/// *This icon requires the feature* `FiLayers` *to be enabled*.
#[component]
pub fn Layers(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2" /><polyline points="2 17 12 22 22 17" /><polyline points="2 12 12 17 22 12" /></svg>
   }
}