#[cfg(feature = "IoInformationCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoInformationCircleSharp")]
/// *This icon requires the feature* `IoInformationCircleSharp` *to be enabled*.
#[component]
pub fn InformationCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,56C145.72,56,56,145.72,56,256s89.72,200,200,200,200-89.72,200-200S366.28,56,256,56Zm0,82a26,26,0,1,1-26,26A26,26,0,0,1,256,138Zm64,226H200V332h44V244H212V212h64V332h44Z" /></svg>
   }
}