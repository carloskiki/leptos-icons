#[cfg(feature = "ImPencil2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPencil2")]
/// *This icon requires the feature* `ImPencil2` *to be enabled*.
#[component]
pub fn Pencil2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6 10l2-1 7-7-1-1-7 7-1 2zM4.52 13.548c-0.494-1.043-1.026-1.574-2.069-2.069l1.548-4.262 2-1.217 6-6h-3l-6 6-3 10 10-3 6-6v-3l-6 6-1.217 2z" /></svg>
   }
}