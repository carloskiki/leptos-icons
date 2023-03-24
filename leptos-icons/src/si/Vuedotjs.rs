#[cfg(feature = "SiVuedotjs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVuedotjs")]
/// *This icon requires the feature* `SiVuedotjs` *to be enabled*.
#[component]
pub fn Vuedotjs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24,1.61H14.06L12,5.16,9.94,1.61H0L12,22.39ZM12,14.08,5.16,2.23H9.59L12,6.41l2.41-4.18h4.43Z" /></svg>
   }
}