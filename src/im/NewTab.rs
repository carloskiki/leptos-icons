#[cfg(feature = "ImNewTab")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImNewTab")]
/// *This icon requires the feature* `ImNewTab` *to be enabled*.
#[component]
pub fn NewTab(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 1v12h12v-12h-12zM14 12h-10v-10h10v10zM2 14v-10.5l-1-1v12.5h12.5l-1-1h-10.5z" /><path fill="#000000" d="M5.5 4l2.5 2.5-3 3 1.5 1.5 3-3 2.5 2.5v-6.5z" /></svg>
   }
}