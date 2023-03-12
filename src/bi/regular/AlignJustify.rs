#[cfg(feature = "BiRegularAlignJustify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlignJustify")]
/// *This icon requires the feature* `BiRegularAlignJustify` *to be enabled*.
#[component]
pub fn AlignJustify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 7h16v2H4zm0-4h16v2H4zm0 8h16v2H4zm0 4h16v2H4zm2 4h12v2H6z" /></svg>
   }
}