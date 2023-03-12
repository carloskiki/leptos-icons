#[cfg(feature = "BiSolidArrowFromRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowFromRight")]
/// *This icon requires the feature* `BiSolidArrowFromRight` *to be enabled*.
#[component]
pub fn ArrowFromRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 6h2v12h-2zm-8 12v-5h6v-2h-6V6l-6 6z" /></svg>
   }
}