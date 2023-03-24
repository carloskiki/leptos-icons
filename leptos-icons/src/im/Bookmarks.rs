#[cfg(feature = "ImBookmarks")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBookmarks")]
/// *This icon requires the feature* `ImBookmarks` *to be enabled*.
#[component]
pub fn Bookmarks(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4 2v14l5-5 5 5v-14zM12 0h-10v14l1-1v-12h9z" /></svg>
   }
}