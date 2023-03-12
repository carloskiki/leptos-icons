#[cfg(feature = "SiHurriyetemlak")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiHurriyetemlak")]
/// *This icon requires the feature* `SiHurriyetemlak` *to be enabled*.
#[component]
pub fn Hurriyetemlak(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 16.085L11.994 4.091 0 16.097l3.817 3.812 8.182-8.189 8.189 8.182z" /></svg>
   }
}