#[cfg(feature = "BiRegularMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMenu")]
/// *This icon requires the feature* `BiRegularMenu` *to be enabled*.
#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h16v2H4zm0 5h16v2H4zm0 5h16v2H4z" /></svg>
   }
}