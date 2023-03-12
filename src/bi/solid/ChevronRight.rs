#[cfg(feature = "BiSolidChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronRight")]
/// *This icon requires the feature* `BiSolidChevronRight` *to be enabled*.
#[component]
pub fn ChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10.061 19.061 17.121 12l-7.06-7.061-2.122 2.122L12.879 12l-4.94 4.939z" /></svg>
   }
}