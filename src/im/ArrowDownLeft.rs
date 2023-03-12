#[cfg(feature = "ImArrowDownLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowDownLeft")]
/// *This icon requires the feature* `ImArrowDownLeft` *to be enabled*.
#[component]
pub fn ArrowDownLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11.5 16l-4-4 8.5-8.5-3.5-3.5-8.5 8.5-4-4v11.5h11.5z" /></svg>
   }
}