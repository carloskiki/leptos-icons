#[cfg(feature = "ImBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBookmark")]
/// *This icon requires the feature* `ImBookmark` *to be enabled*.
#[component]
pub fn Bookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 0v16l5-5 5 5v-16z" /></svg>
   }
}