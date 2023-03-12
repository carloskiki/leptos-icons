#[cfg(feature = "BiSolidLogOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLogOut")]
/// *This icon requires the feature* `BiSolidLogOut` *to be enabled*.
#[component]
pub fn LogOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 2H6a1 1 0 0 0-1 1v9l5-4v3h6v2h-6v3l-5-4v9a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1z" /></svg>
   }
}