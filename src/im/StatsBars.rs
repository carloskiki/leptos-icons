#[cfg(feature = "ImStatsBars")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImStatsBars")]
/// *This icon requires the feature* `ImStatsBars` *to be enabled*.
#[component]
pub fn StatsBars(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 13h16v2h-16zM2 9h2v3h-2zM5 5h2v7h-2zM8 8h2v4h-2zM11 2h2v10h-2z" /></svg>
   }
}