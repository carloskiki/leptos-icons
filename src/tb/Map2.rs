#[cfg(feature = "TbMap2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMap2")]
/// *This icon requires the feature* `TbMap2` *to be enabled*.
#[component]
pub fn Map2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-map-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 6l0 .01" /><path d="M18 13l-3.5 -5a4 4 0 1 1 7 0l-3.5 5" /><path d="M10.5 4.75l-1.5 -.75l-6 3l0 13l6 -3l6 3l6 -3l0 -2" /><path d="M9 4l0 13" /><path d="M15 15l0 5" /></svg>
   }
}