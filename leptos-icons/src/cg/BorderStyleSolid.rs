#[cfg(feature = "CgBorderStyleSolid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBorderStyleSolid")]
/// *This icon requires the feature* `CgBorderStyleSolid` *to be enabled*.
#[component]
pub fn BorderStyleSolid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 11H22V13H2V11Z" fill="currentColor" /></svg>
   }
}