#[cfg(feature = "RiMapFillRoute")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillRoute")]
/// *This icon requires the feature* `RiMapFillRoute` *to be enabled*.
#[component]
pub fn Route(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 15V8.5a4.5 4.5 0 0 1 9 0v7a2.5 2.5 0 1 0 5 0V8.83a3.001 3.001 0 1 1 2 0v6.67a4.5 4.5 0 1 1-9 0v-7a2.5 2.5 0 0 0-5 0V15h3l-4 5-4-5h3z" /></g></svg>
   }
}