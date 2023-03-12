#[cfg(feature = "FaSolidArrowsUpDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidArrowsUpDown")]
/// *This icon requires the feature* `FaSolidArrowsUpDown` *to be enabled*.
#[component]
pub fn ArrowsUpDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 512"><path d="M150.6 9.4c-12.5-12.5-32.8-12.5-45.3 0l-96 96c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L96 109.3V402.7L54.6 361.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l96 96c12.5 12.5 32.8 12.5 45.3 0l96-96c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L160 402.7V109.3l41.4 41.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-96-96z" /></svg>
   }
}