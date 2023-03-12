#[cfg(feature = "RiSystemLineCheckboxIndeterminate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineCheckboxIndeterminate")]
/// *This icon requires the feature* `RiSystemLineCheckboxIndeterminate` *to be enabled*.
#[component]
pub fn CheckboxIndeterminate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm1 2v14h14V5H5zm2 6h10v2H7v-2z" /></g></svg>
   }
}