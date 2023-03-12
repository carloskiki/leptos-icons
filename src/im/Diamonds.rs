#[cfg(feature = "ImDiamonds")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDiamonds")]
/// *This icon requires the feature* `ImDiamonds` *to be enabled*.
#[component]
pub fn Diamonds(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0l-5 8 5 8 5-8z" /></svg>
   }
}