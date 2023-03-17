#[cfg(feature = "HiMdSolidStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidStop")]
/// *This icon requires the feature* `HiMdSolidStop` *to be enabled*.
#[component]
pub fn Stop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M5.25 3C4.00736 3 3 4.00736 3 5.25V14.75C3 15.9926 4.00736 17 5.25 17H14.75C15.9926 17 17 15.9926 17 14.75V5.25C17 4.00736 15.9926 3 14.75 3H5.25Z" fill="#0F172A" /></svg>
   }
}