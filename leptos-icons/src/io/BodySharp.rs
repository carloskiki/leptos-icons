#[cfg(feature = "IoBodySharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBodySharp")]
/// *This icon requires the feature* `IoBodySharp` *to be enabled*.
#[component]
pub fn BodySharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="56" r="56" /><polygon points="464 128 48 128 48 180 192 180 160 505.13 211 512 232.65 320 279.67 320 301 512 352 505.02 320 180 464 180 464 128" /></svg>
   }
}