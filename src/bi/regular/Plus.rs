#[cfg(feature = "BiRegularPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPlus")]
/// *This icon requires the feature* `BiRegularPlus` *to be enabled*.
#[component]
pub fn Plus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 11h-6V5h-2v6H5v2h6v6h2v-6h6z" /></svg>
   }
}