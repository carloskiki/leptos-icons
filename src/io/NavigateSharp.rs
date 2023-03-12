#[cfg(feature = "IoNavigateSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoNavigateSharp")]
/// *This icon requires the feature* `IoNavigateSharp` *to be enabled*.
#[component]
pub fn NavigateSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="480 32 32 240 272 240 272 480 480 32" /></svg>
   }
}