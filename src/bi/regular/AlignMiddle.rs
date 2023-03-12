#[cfg(feature = "BiRegularAlignMiddle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlignMiddle")]
/// *This icon requires the feature* `BiRegularAlignMiddle` *to be enabled*.
#[component]
pub fn AlignMiddle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 19h16v2H4zm3-4h10v2H7zm-3-4h16v2H4zm0-8h16v2H4zm3 4h10v2H7z" /></svg>
   }
}