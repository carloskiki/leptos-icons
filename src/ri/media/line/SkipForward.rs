#[cfg(feature = "RiMediaLineSkipForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLineSkipForward")]
/// *This icon requires the feature* `RiMediaLineSkipForward` *to be enabled*.
#[component]
pub fn SkipForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16 12.667L5.777 19.482A.5.5 0 0 1 5 19.066V4.934a.5.5 0 0 1 .777-.416L16 11.333V5a1 1 0 0 1 2 0v14a1 1 0 0 1-2 0v-6.333zm-9-4.93v8.526L13.394 12 7 7.737z" /></g></svg>
   }
}