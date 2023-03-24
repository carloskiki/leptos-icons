#[cfg(feature = "TbZoomCancel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbZoomCancel")]
/// *This icon requires the feature* `TbZoomCancel` *to be enabled*.
#[component]
pub fn ZoomCancel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-zoom-cancel" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /><path d="M8 8l4 4" /><path d="M12 8l-4 4" /><path d="M21 21l-6 -6" /></svg>
   }
}