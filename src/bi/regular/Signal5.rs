#[cfg(feature = "BiRegularSignal5")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSignal5")]
/// *This icon requires the feature* `BiRegularSignal5` *to be enabled*.
#[component]
pub fn Signal5(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 16h2v5H3zm4-3h2v8H7zm4-3h2v11h-2zm4-3h2v14h-2zm4-3h2v17h-2z" /></svg>
   }
}