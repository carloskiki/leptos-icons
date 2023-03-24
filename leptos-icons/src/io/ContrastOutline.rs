#[cfg(feature = "IoContrastOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoContrastOutline")]
/// *This icon requires the feature* `IoContrastOutline` *to be enabled*.
#[component]
pub fn ContrastOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="208" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M256,464C141.12,464,48,370.88,48,256S141.12,48,256,48Z" /></svg>
   }
}