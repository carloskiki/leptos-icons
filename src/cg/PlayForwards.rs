#[cfg(feature = "CgPlayForwards")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayForwards")]
/// *This icon requires the feature* `CgPlayForwards` *to be enabled*.
#[component]
pub fn PlayForwards(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M21.0023 17H18.0023V7H21.0023V17Z" fill="currentColor" /><path d="M17.0023 12L10 17V7L17.0023 12Z" fill="currentColor" /><path d="M2 17L9.00232 12L2 7V17Z" fill="currentColor" /></svg>
   }
}