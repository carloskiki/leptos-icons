#[cfg(feature = "ImDelicious")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDelicious")]
/// *This icon requires the feature* `ImDelicious` *to be enabled*.
#[component]
pub fn Delicious(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 0v16h16v-16h-16zM8 15v-7h-7v-7h7v7h7v7h-7z" /></svg>
   }
}