#[cfg(feature = "BiSolidMinusSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMinusSquare")]
/// *This icon requires the feature* `BiSolidMinusSquare` *to be enabled*.
#[component]
pub fn MinusSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 3a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5zm12 10H7v-2h10v2z" /></svg>
   }
}