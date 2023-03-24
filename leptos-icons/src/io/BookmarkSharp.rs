#[cfg(feature = "IoBookmarkSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBookmarkSharp")]
/// *This icon requires the feature* `IoBookmarkSharp` *to be enabled*.
#[component]
pub fn BookmarkSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M416,480,256,357.41,96,480V32H416Z" /></svg>
   }
}