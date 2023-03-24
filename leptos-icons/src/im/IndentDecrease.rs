#[cfg(feature = "ImIndentDecrease")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImIndentDecrease")]
/// *This icon requires the feature* `ImIndentDecrease` *to be enabled*.
#[component]
pub fn IndentDecrease(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 1h16v2h-16zM6 4h10v2h-10zM6 7h10v2h-10zM6 10h10v2h-10zM0 13h16v2h-16zM4 5v6l-4-3z" /></svg>
   }
}