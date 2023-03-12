#[cfg(feature = "BiSolidCommentMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCommentMinus")]
/// *This icon requires the feature* `BiSolidCommentMinus` *to be enabled*.
#[component]
pub fn CommentMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-4 9H8V9h8v2z" /></svg>
   }
}