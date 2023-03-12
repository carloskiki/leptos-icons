#[cfg(feature = "OcSmHorizontalRule")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmHorizontalRule")]
/// *This icon requires the feature* `OcSmHorizontalRule` *to be enabled*.
#[component]
pub fn HorizontalRule(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 7.75A.75.75 0 0 1 .75 7h14.5a.75.75 0 0 1 0 1.5H.75A.75.75 0 0 1 0 7.75Z" /></svg>
   }
}