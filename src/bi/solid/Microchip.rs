#[cfg(feature = "BiSolidMicrochip")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMicrochip")]
/// *This icon requires the feature* `BiSolidMicrochip` *to be enabled*.
#[component]
pub fn Microchip(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.999 22h8c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2h-8c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2zm-5-15h2V5h-2v.5h-1v1h1zm18-2h-2v2h2v-.5h1v-1h-1zm-18 6h2V9h-2v.5h-1v1h1zm18-2h-2v2h2v-.5h1v-1h-1zm-18 6h2v-2h-2v.5h-1v1h1zm18-2h-2v2h2v-.5h1v-1h-1zm-18 6h2v-2h-2v.5h-1v1h1zm18-2h-2v2h2v-.5h1v-1h-1z" /></svg>
   }
}