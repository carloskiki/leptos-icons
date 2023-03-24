#[cfg(feature = "ImLibrary")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLibrary")]
/// *This icon requires the feature* `ImLibrary` *to be enabled*.
#[component]
pub fn Library(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="17" height="16" viewBox="0 0 17 16"><path fill="#000000" d="M16 15v-1h-1v-6h1v-1h-3v1h1v6h-3v-6h1v-1h-3v1h1v6h-3v-6h1v-1h-3v1h1v6h-3v-6h1v-1h-3v1h1v6h-1v1h-1v1h17v-1h-1z" /><path fill="#000000" d="M8 0h1l8 5v1h-17v-1l8-5z" /></svg>
   }
}