#[cfg(feature = "CgFramer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgFramer")]
/// *This icon requires the feature* `CgFramer` *to be enabled*.
#[component]
pub fn Framer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 21L12 9L6 9L6 15L12 21Z" fill="currentColor" fill-opacity="0.5" /><path d="M18 9V3H6L12 9H6V15H18L12 9H18Z" fill="currentColor" /></svg>
   }
}