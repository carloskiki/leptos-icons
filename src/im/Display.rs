#[cfg(feature = "ImDisplay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDisplay")]
/// *This icon requires the feature* `ImDisplay` *to be enabled*.
#[component]
pub fn Display(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 1v10h16v-10h-16zM15 10h-14v-8h14v8zM10.5 12h-5l-0.5 2-1 1h8l-1-1z" /></svg>
   }
}