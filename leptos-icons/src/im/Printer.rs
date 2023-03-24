#[cfg(feature = "ImPrinter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPrinter")]
/// *This icon requires the feature* `ImPrinter` *to be enabled*.
#[component]
pub fn Printer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4 1h8v2h-8v-2z" /><path fill="#000000" d="M15 4h-14c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h3v4h8v-4h3c0.55 0 1-0.45 1-1v-5c0-0.55-0.45-1-1-1zM2 7c-0.552 0-1-0.448-1-1s0.448-1 1-1 1 0.448 1 1-0.448 1-1 1zM11 14h-6v-5h6v5z" /></svg>
   }
}