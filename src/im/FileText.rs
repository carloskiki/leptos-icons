#[cfg(feature = "ImFileText")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFileText")]
/// *This icon requires the feature* `ImFileText` *to be enabled*.
#[component]
pub fn FileText(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.5 0h-12c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h12c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM13 14h-11v-12h11v12zM4 7h7v1h-7zM4 9h7v1h-7zM4 11h7v1h-7zM4 5h7v1h-7z" /></svg>
   }
}