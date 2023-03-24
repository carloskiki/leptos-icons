#[cfg(feature = "CgTerrain")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgTerrain")]
/// *This icon requires the feature* `CgTerrain` *to be enabled*.
#[component]
pub fn Terrain(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M8 10L3 18H13L8 10Z" fill="currentColor" /><path d="M10.5286 10.7543L13.5 6L21 18H15.0572L10.5286 10.7543Z" fill="currentColor" /></svg>
   }
}