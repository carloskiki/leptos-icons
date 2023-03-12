#[cfg(feature = "ImUpload3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImUpload3")]
/// *This icon requires the feature* `ImUpload3` *to be enabled*.
#[component]
pub fn Upload3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M7.5 11h-7.5v4h15v-4h-7.5zM14 13h-2v-1h2v1zM3.5 5l4-4 4 4h-2.5v5h-3v-5z" /></svg>
   }
}