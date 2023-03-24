#[cfg(feature = "OcSmDotFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmDotFill")]
/// *This icon requires the feature* `OcSmDotFill` *to be enabled*.
#[component]
pub fn DotFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 4a4 4 0 1 1 0 8 4 4 0 0 1 0-8Z" /></svg>
   }
}