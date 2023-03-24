#[cfg(feature = "BsRecord2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsRecord2")]
/// *This icon requires the feature* `BsRecord2` *to be enabled*.
#[component]
pub fn Record2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-record2" viewBox="0 0 16 16"><path d="M8 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0 1A5 5 0 1 0 8 3a5 5 0 0 0 0 10z" /><path d="M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /></svg>
   }
}