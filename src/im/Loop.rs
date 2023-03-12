#[cfg(feature = "ImLoop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLoop")]
/// *This icon requires the feature* `ImLoop` *to be enabled*.
#[component]
pub fn Loop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M2 5h10v3l4-4-4-4v3h-12v6h2zM14 11h-10v-3l-4 4 4 4v-3h12v-6h-2z" /></svg>
   }
}