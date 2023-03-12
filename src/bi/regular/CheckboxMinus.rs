#[cfg(feature = "BiRegularCheckboxMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCheckboxMinus")]
/// *This icon requires the feature* `BiRegularCheckboxMinus` *to be enabled*.
#[component]
pub fn CheckboxMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9.01 11h6v2h-6z" /><path d="M17 5H7a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2zM7 17V7h10v10z" /></svg>
   }
}