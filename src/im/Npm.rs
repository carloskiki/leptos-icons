#[cfg(feature = "ImNpm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImNpm")]
/// *This icon requires the feature* `ImNpm` *to be enabled*.
#[component]
pub fn Npm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 0v16h16v-16h-16zM13 13h-2v-8h-3v8h-5v-10h10v10z" /></svg>
   }
}