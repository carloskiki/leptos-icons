#[cfg(feature = "SiVerizon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVerizon")]
/// *This icon requires the feature* `SiVerizon` *to be enabled*.
#[component]
pub fn Verizon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M18.302 0H22v.003L10.674 24H7.662L2 12h3.727l3.449 7.337z" /></svg>
   }
}