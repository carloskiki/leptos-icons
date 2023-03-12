#[cfg(feature = "SiVitess")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVitess")]
/// *This icon requires the feature* `SiVitess` *to be enabled*.
#[component]
pub fn Vitess(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="m19.206 1.045-7.217 13.186L4.817 1.045H0l11.904 21.91L24 1.045h-4.794Z" /></svg>
   }
}