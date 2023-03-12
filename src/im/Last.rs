#[cfg(feature = "ImLast")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLast")]
/// *This icon requires the feature* `ImLast` *to be enabled*.
#[component]
pub fn Last(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 2v12h-2v-5.5l-5 5v-5l-5 5v-11l5 5v-5l5 5v-5.5z" /></svg>
   }
}