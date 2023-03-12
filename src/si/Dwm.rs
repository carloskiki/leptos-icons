#[cfg(feature = "SiDwm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDwm")]
/// *This icon requires the feature* `SiDwm` *to be enabled*.
#[component]
pub fn Dwm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 11h6V7h2v8h2v-4h2v4h2v-4h10v6h-2v-4h-2v4h-2v-4h-2v4H2v-2h4v-2H2v4H0z" /></svg>
   }
}