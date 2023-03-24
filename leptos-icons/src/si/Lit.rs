#[cfg(feature = "SiLit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLit")]
/// *This icon requires the feature* `SiLit` *to be enabled*.
#[component]
pub fn Lit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M2.4 9.6l4.8 4.8V24l-4.8-4.8V9.6zm4.8-4.8v9.6L12 9.6V0L7.2 4.8zM12 9.6v9.6l4.8-4.8V4.8L12 9.6zm4.8 4.8V24l4.8-4.8V9.6l-4.8 4.8z" /></svg>
   }
}