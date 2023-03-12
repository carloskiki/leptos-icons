#[cfg(feature = "TbMapPin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMapPin")]
/// *This icon requires the feature* `TbMapPin` *to be enabled*.
#[component]
pub fn MapPin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-map-pin" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 11m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0z" /></svg>
   }
}