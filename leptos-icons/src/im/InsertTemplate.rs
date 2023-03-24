#[cfg(feature = "ImInsertTemplate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImInsertTemplate")]
/// *This icon requires the feature* `ImInsertTemplate` *to be enabled*.
#[component]
pub fn InsertTemplate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6 3h2v1h-2zM9 3h2v1h-2zM14 3v4h-3v-1h2v-2h-1v-1zM5 6h2v1h-2zM8 6h2v1h-2zM3 4v2h1v1h-2v-4h3v1zM6 9h2v1h-2zM9 9h2v1h-2zM14 9v4h-3v-1h2v-2h-1v-1zM5 12h2v1h-2zM8 12h2v1h-2zM3 10v2h1v1h-2v-4h3v1zM15 1h-14v14h14v-14zM16 0v0 16h-16v-16h16z" /></svg>
   }
}