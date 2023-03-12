#[cfg(feature = "IoVideocamSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoVideocamSharp")]
/// *This icon requires the feature* `IoVideocamSharp` *to be enabled*.
#[component]
pub fn VideocamSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M336,208V128a16,16,0,0,0-16-16H32a16,16,0,0,0-16,16V384a16,16,0,0,0,16,16H320a16,16,0,0,0,16-16V304l160,96V112Z" /></svg>
   }
}