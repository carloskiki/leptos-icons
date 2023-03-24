#[cfg(feature = "ImPower")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPower")]
/// *This icon requires the feature* `ImPower` *to be enabled*.
#[component]
pub fn Power(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6 0l-6 8h6l-4 8 14-10h-8l6-6z" /></svg>
   }
}