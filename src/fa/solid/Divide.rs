#[cfg(feature = "FaSolidDivide")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidDivide")]
/// *This icon requires the feature* `FaSolidDivide` *to be enabled*.
#[component]
pub fn Divide(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M256 96a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm0 320a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM384 288c17.7 0 32-14.3 32-32s-14.3-32-32-32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H384z" /></svg>
   }
}