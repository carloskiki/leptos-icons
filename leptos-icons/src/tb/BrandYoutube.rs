#[cfg(feature = "TbBrandYoutube")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandYoutube")]
/// *This icon requires the feature* `TbBrandYoutube` *to be enabled*.
#[component]
pub fn BrandYoutube(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-youtube" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 5m0 4a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v6a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4z" /><path d="M10 9l5 3l-5 3z" /></svg>
   }
}