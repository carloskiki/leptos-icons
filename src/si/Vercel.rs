#[cfg(feature = "SiVercel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVercel")]
/// *This icon requires the feature* `SiVercel` *to be enabled*.
#[component]
pub fn Vercel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 22.525H0l12-21.05 12 21.05z" /></svg>
   }
}