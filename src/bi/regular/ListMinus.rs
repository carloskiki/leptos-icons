#[cfg(feature = "BiRegularListMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularListMinus")]
/// *This icon requires the feature* `BiRegularListMinus` *to be enabled*.
#[component]
pub fn ListMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21.063 15H13v2h9v-2zM4 7h11v2H4zm0 4h11v2H4zm0 4h7v2H4z" /></svg>
   }
}