#[cfg(feature = "SiLospec")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLospec")]
/// *This icon requires the feature* `SiLospec` *to be enabled*.
#[component]
pub fn Lospec(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M4.23 0v24h15.541v-8.4004h-7.1719v3.5996H11.402V0z" /></svg>
   }
}