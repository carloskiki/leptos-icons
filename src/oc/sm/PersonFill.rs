#[cfg(feature = "OcSmPersonFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmPersonFill")]
/// *This icon requires the feature* `OcSmPersonFill` *to be enabled*.
#[component]
pub fn PersonFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M4.243 4.757a3.757 3.757 0 1 1 5.851 3.119 6.006 6.006 0 0 1 3.9 5.339.75.75 0 0 1-.715.784H2.721a.75.75 0 0 1-.714-.784 6.006 6.006 0 0 1 3.9-5.34 3.753 3.753 0 0 1-1.664-3.118Z" /></svg>
   }
}