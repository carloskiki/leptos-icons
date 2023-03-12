#[cfg(feature = "ImTextColor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTextColor")]
/// *This icon requires the feature* `ImTextColor` *to be enabled*.
#[component]
pub fn TextColor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M5.032 13l0.9-3h4.137l0.9 3h1.775l-3-10h-3.488l-3 10h1.776zM7.432 5h1.137l0.9 3h-2.937l0.9-3z" /></svg>
   }
}