#[cfg(feature = "ImFolder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolder")]
/// *This icon requires the feature* `ImFolder` *to be enabled*.
#[component]
pub fn Folder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M7 2l2 2h7v11h-16v-13z" /></svg>
   }
}