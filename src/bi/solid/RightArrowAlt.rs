#[cfg(feature = "BiSolidRightArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRightArrowAlt")]
/// *This icon requires the feature* `BiSolidRightArrowAlt` *to be enabled*.
#[component]
pub fn RightArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m19 12-7-6v5H6v2h6v5z" /></svg>
   }
}