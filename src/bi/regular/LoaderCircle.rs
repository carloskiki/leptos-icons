#[cfg(feature = "BiRegularLoaderCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLoaderCircle")]
/// *This icon requires the feature* `BiRegularLoaderCircle` *to be enabled*.
#[component]
pub fn LoaderCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="20" r="2" /><circle cx="12" cy="4" r="2" /><circle cx="6.343" cy="17.657" r="2" /><circle cx="17.657" cy="6.343" r="2" /><circle cx="4" cy="12" r="2.001" /><circle cx="20" cy="12" r="2" /><circle cx="6.343" cy="6.344" r="2" /><circle cx="17.657" cy="17.658" r="2" /></svg>
   }
}