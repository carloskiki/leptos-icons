#[cfg(feature = "BiSolidBookmarkMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookmarkMinus")]
/// *This icon requires the feature* `BiSolidBookmarkMinus` *to be enabled*.
#[component]
pub fn BookmarkMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 2H7a2 2 0 0 0-2 2v18l7-4.848L19 22V4a2 2 0 0 0-2-2zm-1 9H8V9h8v2z" /></svg>
   }
}