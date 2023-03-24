#[cfg(feature = "ImRoad")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImRoad")]
/// *This icon requires the feature* `ImRoad` *to be enabled*.
#[component]
pub fn Road(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11 16h5l-4-16h-3l0.5 4h-3l0.5-4h-3l-4 16h5l0.5-4h5l0.5 4zM5.75 10l0.5-4h3.5l0.5 4h-4.5z" /></svg>
   }
}