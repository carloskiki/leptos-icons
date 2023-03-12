#[cfg(feature = "BiRegularSpaceBar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSpaceBar")]
/// *This icon requires the feature* `BiRegularSpaceBar` *to be enabled*.
#[component]
pub fn SpaceBar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 13H7V9H5v6h14V9h-2z" /></svg>
   }
}