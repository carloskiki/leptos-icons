#[cfg(feature = "BiSolidChevronsUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronsUp")]
/// *This icon requires the feature* `BiSolidChevronsUp` *to be enabled*.
#[component]
pub fn ChevronsUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 3.879-7.061 7.06 2.122 2.122L12 8.121l4.939 4.94 2.122-2.122z" /><path d="m4.939 17.939 2.122 2.122L12 15.121l4.939 4.94 2.122-2.122L12 10.879z" /></svg>
   }
}