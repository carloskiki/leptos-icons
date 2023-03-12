#[cfg(feature = "FaSolidNeuter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidNeuter")]
/// *This icon requires the feature* `FaSolidNeuter` *to be enabled*.
#[component]
pub fn Neuter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M64 176a112 112 0 1 1 224 0A112 112 0 1 1 64 176zM208 349.1c81.9-15 144-86.8 144-173.1C352 78.8 273.2 0 176 0S0 78.8 0 176c0 86.3 62.1 158.1 144 173.1V480c0 17.7 14.3 32 32 32s32-14.3 32-32V349.1z" /></svg>
   }
}