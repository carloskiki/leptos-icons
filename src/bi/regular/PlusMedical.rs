#[cfg(feature = "BiRegularPlusMedical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPlusMedical")]
/// *This icon requires the feature* `BiRegularPlusMedical` *to be enabled*.
#[component]
pub fn PlusMedical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 2.013H9V9H2v6h7v6.987h6V15h7V9h-7z" /></svg>
   }
}