#[cfg(feature = "RiMapFillMap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillMap")]
/// *This icon requires the feature* `RiMapFillMap` *to be enabled*.
#[component]
pub fn Map(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 5l7-3 6 3 6.303-2.701a.5.5 0 0 1 .697.46V19l-7 3-6-3-6.303 2.701a.5.5 0 0 1-.697-.46V5z" /></g></svg>
   }
}