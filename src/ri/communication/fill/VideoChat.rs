#[cfg(feature = "RiCommunicationFillVideoChat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationFillVideoChat")]
/// *This icon requires the feature* `RiCommunicationFillVideoChat` *to be enabled*.
#[component]
pub fn VideoChat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zM14 10.25V8H7v6h7v-2.25L17 14V8l-3 2.25z" /></g></svg>
   }
}