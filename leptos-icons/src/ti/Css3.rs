#[cfg(feature = "TiCss3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiCss3")]
/// *This icon requires the feature* `TiCss3` *to be enabled*.
#[component]
pub fn Css3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1" width="24" height="24" viewBox="0 0 24 24"><path d="M5.7 3.4l-.6 3.2h12.3l-.4 2.1h-12.3l-.6 3.2h12.3l-.7 3.6-5 1.7-4.3-1.7.3-1.6h-3l-.7 3.8 7.1 2.9 8.2-2.9 1.1-5.8.2-1.2 1.4-7.3h-15.3z" /></svg>
   }
}