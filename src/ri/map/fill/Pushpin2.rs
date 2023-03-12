#[cfg(feature = "RiMapFillPushpin2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillPushpin2")]
/// *This icon requires the feature* `RiMapFillPushpin2` *to be enabled*.
#[component]
pub fn Pushpin2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M18 3v2h-1v6l2 3v2h-6v7h-2v-7H5v-2l2-3V5H6V3z" /></g></svg>
   }
}