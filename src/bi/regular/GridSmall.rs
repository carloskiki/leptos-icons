#[cfg(feature = "BiRegularGridSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularGridSmall")]
/// *This icon requires the feature* `BiRegularGridSmall` *to be enabled*.
#[component]
pub fn GridSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 7h4v4H7zm0 6h4v4H7zm6-6h4v4h-4zm0 6h4v4h-4z" /></svg>
   }
}