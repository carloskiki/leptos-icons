#[cfg(feature = "BiRegularCaretRightSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCaretRightSquare")]
/// *This icon requires the feature* `BiRegularCaretRightSquare` *to be enabled*.
#[component]
pub fn CaretRightSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m10 17 6-5-6-5z" /><path d="M19 3H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM5 19V5h14l.002 14H5z" /></svg>
   }
}