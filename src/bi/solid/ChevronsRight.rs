#[cfg(feature = "BiSolidChevronsRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronsRight")]
/// *This icon requires the feature* `BiSolidChevronsRight` *to be enabled*.
#[component]
pub fn ChevronsRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m13.061 4.939-2.122 2.122L15.879 12l-4.94 4.939 2.122 2.122L20.121 12z" /><path d="M6.061 19.061 13.121 12l-7.06-7.061-2.122 2.122L8.879 12l-4.94 4.939z" /></svg>
   }
}