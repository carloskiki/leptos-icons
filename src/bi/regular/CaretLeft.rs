#[cfg(feature = "BiRegularCaretLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCaretLeft")]
/// *This icon requires the feature* `BiRegularCaretLeft` *to be enabled*.
#[component]
pub fn CaretLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 19V5l-8 7z" /></svg>
   }
}