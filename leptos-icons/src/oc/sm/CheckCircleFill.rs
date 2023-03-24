#[cfg(feature = "OcSmCheckCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmCheckCircleFill")]
/// *This icon requires the feature* `OcSmCheckCircleFill` *to be enabled*.
#[component]
pub fn CheckCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16Zm3.78-9.72a.751.751 0 0 0-.018-1.042.751.751 0 0 0-1.042-.018L6.75 9.19 5.28 7.72a.751.751 0 0 0-1.042.018.751.751 0 0 0-.018 1.042l2 2a.75.75 0 0 0 1.06 0Z" /></svg>
   }
}