#[cfg(feature = "RiSystemFillUpload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillUpload")]
/// *This icon requires the feature* `RiSystemFillUpload` *to be enabled*.
#[component]
pub fn Upload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 19h18v2H3v-2zm10-9v8h-2v-8H4l8-8 8 8h-7z" /></g></svg>
   }
}