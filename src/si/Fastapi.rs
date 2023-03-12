#[cfg(feature = "SiFastapi")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFastapi")]
/// *This icon requires the feature* `SiFastapi` *to be enabled*.
#[component]
pub fn Fastapi(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0C5.375 0 0 5.375 0 12c0 6.627 5.375 12 12 12 6.626 0 12-5.373 12-12 0-6.625-5.373-12-12-12zm-.624 21.62v-7.528H7.19L13.203 2.38v7.528h4.029L11.376 21.62z" /></svg>
   }
}