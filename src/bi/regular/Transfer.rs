#[cfg(feature = "BiRegularTransfer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTransfer")]
/// *This icon requires the feature* `BiRegularTransfer` *to be enabled*.
#[component]
pub fn Transfer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m15 12 5-4-5-4v2.999H2v2h13zm7 3H9v-3l-5 4 5 4v-3h13z" /></svg>
   }
}