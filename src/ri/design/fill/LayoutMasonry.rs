#[cfg(feature = "RiDesignFillLayoutMasonry")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillLayoutMasonry")]
/// *This icon requires the feature* `RiDesignFillLayoutMasonry` *to be enabled*.
#[component]
pub fn LayoutMasonry(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M22 9.999V20a1 1 0 0 1-1 1h-8V9.999h9zm-11 6V21H3a1 1 0 0 1-1-1v-4.001h9zM11 3v10.999H2V4a1 1 0 0 1 1-1h8zm10 0a1 1 0 0 1 1 1v3.999h-9V3h8z" /></g></svg>
   }
}