#[cfg(feature = "SiAwsamplify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAwsamplify")]
/// *This icon requires the feature* `SiAwsamplify` *to be enabled*.
#[component]
pub fn Awsamplify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M5.223 17.905h6.76l1.731 3.047H0l4.815-8.344 2.018-3.494 1.733 3.002zm2.52-10.371L9.408 4.65l9.415 16.301h-3.334zm2.59-4.486h3.33L24 20.952h-3.334z" /></svg>
   }
}