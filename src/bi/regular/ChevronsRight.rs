#[cfg(feature = "BiRegularChevronsRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronsRight")]
/// *This icon requires the feature* `BiRegularChevronsRight` *to be enabled*.
#[component]
pub fn ChevronsRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10.296 7.71 14.621 12l-4.325 4.29 1.408 1.42L17.461 12l-5.757-5.71z" /><path d="M6.704 6.29 5.296 7.71 9.621 12l-4.325 4.29 1.408 1.42L12.461 12z" /></svg>
   }
}