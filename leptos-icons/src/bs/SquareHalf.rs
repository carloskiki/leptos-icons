#[cfg(feature = "BsSquareHalf")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsSquareHalf")]
/// *This icon requires the feature* `BsSquareHalf` *to be enabled*.
#[component]
pub fn SquareHalf(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-square-half" viewBox="0 0 16 16"><path d="M8 15V1h6a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H8zm6 1a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12z" /></svg>
   }
}