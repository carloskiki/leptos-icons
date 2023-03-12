#[cfg(feature = "BiRegularSubdirectoryRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSubdirectoryRight")]
/// *This icon requires the feature* `BiRegularSubdirectoryRight` *to be enabled*.
#[component]
pub fn SubdirectoryRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M14 13H8V5H6v9a1 1 0 0 0 1 1h7v3l5-4-5-4v3z" /></svg>
   }
}