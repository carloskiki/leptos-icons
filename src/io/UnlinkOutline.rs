#[cfg(feature = "IoUnlinkOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoUnlinkOutline")]
/// *This icon requires the feature* `IoUnlinkOutline` *to be enabled*.
#[component]
pub fn UnlinkOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><path d="M208,352H144a96,96,0,0,1,0-192h64" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="36" /><path d="M304,160h64a96,96,0,0,1,0,192H304" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="36" /></svg>
   }
}