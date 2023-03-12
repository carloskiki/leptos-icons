#[cfg(feature = "BiRegularSubdirectoryLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSubdirectoryLeft")]
/// *This icon requires the feature* `BiRegularSubdirectoryLeft` *to be enabled*.
#[component]
pub fn SubdirectoryLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 13h-6v-3l-5 4 5 4v-3h7a1 1 0 0 0 1-1V5h-2v8z" /></svg>
   }
}