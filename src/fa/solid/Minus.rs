#[cfg(feature = "FaSolidMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMinus")]
/// *This icon requires the feature* `FaSolidMinus` *to be enabled*.
#[component]
pub fn Minus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M416 256c0 17.7-14.3 32-32 32L32 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l352 0c17.7 0 32 14.3 32 32z" /></svg>
   }
}