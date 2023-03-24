#[cfg(feature = "SiStackblitz")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiStackblitz")]
/// *This icon requires the feature* `SiStackblitz` *to be enabled*.
#[component]
pub fn Stackblitz(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M10.797 14.182H3.635L16.728 0l-3.525 9.818h7.162L7.272 24l3.524-9.818Z" /></svg>
   }
}