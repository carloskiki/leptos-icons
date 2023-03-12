#[cfg(feature = "RiDesignFillLayoutRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillLayoutRight")]
/// *This icon requires the feature* `RiDesignFillLayoutRight` *to be enabled*.
#[component]
pub fn LayoutRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 3a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-4V3h4zm-6 18H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h12v18z" /></g></svg>
   }
}