#[cfg(feature = "BiRegularCaretDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCaretDown")]
/// *This icon requires the feature* `BiRegularCaretDown` *to be enabled*.
#[component]
pub fn CaretDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m11.998 17 7-8h-14z" /></svg>
   }
}