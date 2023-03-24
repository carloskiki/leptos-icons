#[cfg(feature = "TbBrandVue")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandVue")]
/// *This icon requires the feature* `TbBrandVue` *to be enabled*.
#[component]
pub fn BrandVue(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-vue" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M16.5 4l-4.5 8l-4.5 -8" /><path d="M3 4l9 16l9 -16" /></svg>
   }
}