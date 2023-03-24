#[cfg(feature = "IoStopSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoStopSharp")]
/// *This icon requires the feature* `IoStopSharp` *to be enabled*.
#[component]
pub fn StopSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="80" y="80" width="352" height="352" /></svg>
   }
}