#[cfg(feature = "BiRegularCaretRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCaretRight")]
/// *This icon requires the feature* `BiRegularCaretRight` *to be enabled*.
#[component]
pub fn CaretRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m9 19 8-7-8-7z" /></svg>
   }
}