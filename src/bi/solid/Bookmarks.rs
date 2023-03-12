#[cfg(feature = "BiSolidBookmarks")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookmarks")]
/// *This icon requires the feature* `BiSolidBookmarks` *to be enabled*.
#[component]
pub fn Bookmarks(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.999 23V7c0-1.103-.897-2-2-2h-8c-1.103 0-2 .897-2 2v16l6-3.601 6 3.601z" /><path d="M15.585 3h1.414c1.103 0 2 .897 2 2v10.443l2 2.489V3c0-1.103-.897-2-2-2h-8c-1.103 0-2 .897-2 2h6.586z" /></svg>
   }
}