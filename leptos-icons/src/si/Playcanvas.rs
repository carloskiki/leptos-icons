#[cfg(feature = "SiPlaycanvas")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPlaycanvas")]
/// *This icon requires the feature* `SiPlaycanvas` *to be enabled*.
#[component]
pub fn Playcanvas(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M6.115 0l-.002 3.414 5.823 3.41-5.82 3.414-.003 3.412 11.774-6.826zm11.77 10.35L6.113 17.174 17.887 24l-.002-3.414-5.82-3.412 5.822-3.412z" /></svg>
   }
}