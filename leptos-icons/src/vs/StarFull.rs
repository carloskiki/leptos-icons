#[cfg(feature = "VsStarFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsStarFull")]
/// *This icon requires the feature* `VsStarFull` *to be enabled*.
#[component]
pub fn StarFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M9.595 6.252L8 1 6.405 6.252H1l4.373 3.4L3.75 15 8 11.695 12.25 15l-1.623-5.348L15 6.252H9.595z" /></svg>
   }
}