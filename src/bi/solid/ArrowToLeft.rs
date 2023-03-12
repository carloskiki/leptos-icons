#[cfg(feature = "BiSolidArrowToLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowToLeft")]
/// *This icon requires the feature* `BiSolidArrowToLeft` *to be enabled*.
#[component]
pub fn ArrowToLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h2v12H4zm10 7h6v-2h-6V6l-6 6 6 6z" /></svg>
   }
}