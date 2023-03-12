#[cfg(feature = "RiMapFillGuide")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillGuide")]
/// *This icon requires the feature* `RiMapFillGuide` *to be enabled*.
#[component]
pub fn Guide(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13 8v8a3 3 0 0 1-3 3H7.83a3.001 3.001 0 1 1 0-2H10a1 1 0 0 0 1-1V8a3 3 0 0 1 3-3h3V2l5 4-5 4V7h-3a1 1 0 0 0-1 1z" /></g></svg>
   }
}