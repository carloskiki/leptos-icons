#[cfg(feature = "ImQuill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImQuill")]
/// *This icon requires the feature* `ImQuill` *to be enabled*.
#[component]
pub fn Quill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 16c2-6 7.234-16 16-16-4.109 3.297-6 11-9 11s-3 0-3 0l-3 5h-1z" /></svg>
   }
}