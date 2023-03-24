#[cfg(feature = "ImUpload2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImUpload2")]
/// *This icon requires the feature* `ImUpload2` *to be enabled*.
#[component]
pub fn Upload2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 14h16v1h-16zM16 12v1h-16v-1l2-4h4v2h4v-2h4zM3.5 5l4.5-4.5 4.5 4.5h-3.5v4h-2v-4z" /></svg>
   }
}