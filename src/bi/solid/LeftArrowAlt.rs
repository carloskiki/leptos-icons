#[cfg(feature = "BiSolidLeftArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLeftArrowAlt")]
/// *This icon requires the feature* `BiSolidLeftArrowAlt` *to be enabled*.
#[component]
pub fn LeftArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m5 12 7 6v-5h6v-2h-6V6z" /></svg>
   }
}