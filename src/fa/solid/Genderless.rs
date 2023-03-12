#[cfg(feature = "FaSolidGenderless")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidGenderless")]
/// *This icon requires the feature* `FaSolidGenderless` *to be enabled*.
#[component]
pub fn Genderless(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M176 144a112 112 0 1 1 0 224 112 112 0 1 1 0-224zm0 288a176 176 0 1 0 0-352 176 176 0 1 0 0 352z" /></svg>
   }
}