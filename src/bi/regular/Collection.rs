#[cfg(feature = "BiRegularCollection")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCollection")]
/// *This icon requires the feature* `BiRegularCollection` *to be enabled*.
#[component]
pub fn Collection(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 10H5c-1.103 0-2 .897-2 2v8c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2v-8c0-1.103-.897-2-2-2zM5 20v-8h14l.002 8H5zM5 6h14v2H5zm2-4h10v2H7z" /></svg>
   }
}