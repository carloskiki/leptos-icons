#[cfg(feature = "BsEggFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsEggFill")]
/// *This icon requires the feature* `BsEggFill` *to be enabled*.
#[component]
pub fn EggFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-egg-fill" viewBox="0 0 16 16"><path d="M14 10a6 6 0 0 1-12 0C2 5.686 5 0 8 0s6 5.686 6 10z" /></svg>
   }
}