#[cfg(feature = "SiIstio")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiIstio")]
/// *This icon requires the feature* `SiIstio` *to be enabled*.
#[component]
pub fn Istio(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M4 21 20 21 10 24zM4 20 10 19 10 8zM11 19 20 20 11 0z" /></svg>
   }
}