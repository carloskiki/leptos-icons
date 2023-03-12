#[cfg(feature = "BiRegularMenuAltRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMenuAltRight")]
/// *This icon requires the feature* `BiRegularMenuAltRight` *to be enabled*.
#[component]
pub fn MenuAltRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h16v2H4zm4 5h12v2H8zm5 5h7v2h-7z" /></svg>
   }
}