#[cfg(feature = "RiSystemFillCheckboxBlankCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillCheckboxBlankCircle")]
/// *This icon requires the feature* `RiSystemFillCheckboxBlankCircle` *to be enabled*.
#[component]
pub fn CheckboxBlankCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><circle cx="12" cy="12" r="10" /></g></svg>
   }
}