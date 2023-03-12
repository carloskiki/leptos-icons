#[cfg(feature = "TbBrandPushbullet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandPushbullet")]
/// *This icon requires the feature* `TbBrandPushbullet` *to be enabled*.
#[component]
pub fn BrandPushbullet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-pushbullet" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M11 8v8h2a4 4 0 1 0 0 -8h-2z" /><path d="M8 8v8" /></svg>
   }
}