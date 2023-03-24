#[cfg(feature = "ImList")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImList")]
/// *This icon requires the feature* `ImList` *to be enabled*.
#[component]
pub fn List(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 0h4v4h-4zM6 1h10v2h-10zM0 6h4v4h-4zM6 7h10v2h-10zM0 12h4v4h-4zM6 13h10v2h-10z" /></svg>
   }
}