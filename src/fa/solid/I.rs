#[cfg(feature = "FaSolidI")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidI")]
/// *This icon requires the feature* `FaSolidI` *to be enabled*.
#[component]
pub fn I(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M32 32C14.3 32 0 46.3 0 64S14.3 96 32 96h96V416H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H288c17.7 0 32-14.3 32-32s-14.3-32-32-32H192V96h96c17.7 0 32-14.3 32-32s-14.3-32-32-32H160 32z" /></svg>
   }
}