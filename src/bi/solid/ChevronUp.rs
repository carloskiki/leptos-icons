#[cfg(feature = "BiSolidChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronUp")]
/// *This icon requires the feature* `BiSolidChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 6.879-7.061 7.06 2.122 2.122L12 11.121l4.939 4.94 2.122-2.122z" /></svg>
   }
}