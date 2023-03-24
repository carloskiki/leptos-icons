#[cfg(feature = "SiPeertube")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPeertube")]
/// *This icon requires the feature* `SiPeertube` *to be enabled*.
#[component]
pub fn Peertube(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 6.545v10.91L20.727 12M3.273 12v12L12 17.455M3.273 0v12L12 6.545" /></svg>
   }
}