#[cfg(feature = "FaSolidCircleHalfStroke")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidCircleHalfStroke")]
/// *This icon requires the feature* `FaSolidCircleHalfStroke` *to be enabled*.
#[component]
pub fn CircleHalfStroke(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M448 256c0-106-86-192-192-192V448c106 0 192-86 192-192zm64 0c0 141.4-114.6 256-256 256S0 397.4 0 256S114.6 0 256 0S512 114.6 512 256z" /></svg>
   }
}