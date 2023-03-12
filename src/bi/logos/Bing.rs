#[cfg(feature = "BiLogosBing")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosBing")]
/// *This icon requires the feature* `BiLogosBing` *to be enabled*.
#[component]
pub fn Bing(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m5.71 3 3.593 1.264v12.645l5.061-2.919-2.48-1.165-1.566-3.897 7.974 2.802v4.073l-8.984 5.183-3.595-2L5.71 3z" /></svg>
   }
}