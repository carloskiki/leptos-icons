#[cfg(feature = "SiBookmeter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBookmeter")]
/// *This icon requires the feature* `SiBookmeter` *to be enabled*.
#[component]
pub fn Bookmeter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M.678 14.262h6.089V24H.678v-9.738zm8.215 9.717h6.089V7.11H8.893v16.869zM17.234 0v24h6.089V0h-6.089z" /></svg>
   }
}