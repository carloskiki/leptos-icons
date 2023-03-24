#[cfg(feature = "TbArrowRoundaboutRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowRoundaboutRight")]
/// *This icon requires the feature* `TbArrowRoundaboutRight` *to be enabled*.
#[component]
pub fn ArrowRoundaboutRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-roundabout-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 9h-8a5 5 0 1 0 -5 5v7" /><path d="M17 5l4 4l-4 4" /></svg>
   }
}