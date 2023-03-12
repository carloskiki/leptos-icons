#[cfg(feature = "BiRegularCaretUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCaretUp")]
/// *This icon requires the feature* `BiRegularCaretUp` *to be enabled*.
#[component]
pub fn CaretUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 15h14l-7-8z" /></svg>
   }
}