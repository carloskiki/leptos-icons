#[cfg(feature = "SiDeviantart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDeviantart")]
/// *This icon requires the feature* `SiDeviantart` *to be enabled*.
#[component]
pub fn Deviantart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.207 4.794l.23-.43V0H15.07l-.436.44-2.058 3.925-.646.436H4.58v5.993h4.04l.36.436-4.175 7.98-.24.43V24H8.93l.436-.44 2.07-3.925.644-.436h7.35v-5.993h-4.05l-.36-.438 4.186-7.977z" /></svg>
   }
}