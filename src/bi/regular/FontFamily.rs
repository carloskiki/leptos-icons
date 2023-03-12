#[cfg(feature = "BiRegularFontFamily")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFontFamily")]
/// *This icon requires the feature* `BiRegularFontFamily` *to be enabled*.
#[component]
pub fn FontFamily(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 4h7v2h-7zm1 4h6v2h-6zm2 4h4v2h-4zM9.307 4l-6 16h2.137l1.875-5h6.363l1.875 5h2.137l-6-16H9.307zm-1.239 9L10.5 6.515 12.932 13H8.068z" /></svg>
   }
}