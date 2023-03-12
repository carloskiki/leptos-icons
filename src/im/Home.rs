#[cfg(feature = "ImHome")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImHome")]
/// *This icon requires the feature* `ImHome` *to be enabled*.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 9.226l-8-6.21-8 6.21v-2.532l8-6.21 8 6.21zM14 9v6h-4v-4h-4v4h-4v-6l6-4.5z" /></svg>
   }
}