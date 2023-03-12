#[cfg(feature = "BiSolidDirections")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDirections")]
/// *This icon requires the feature* `BiSolidDirections` *to be enabled*.
#[component]
pub fn Directions(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 11h-6V8h6a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H5L2 5l3 3h6v3H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h6v5h2v-5h6l3-3-3-3z" /></svg>
   }
}