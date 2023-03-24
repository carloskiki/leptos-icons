#[cfg(feature = "SiNetapp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiNetapp")]
/// *This icon requires the feature* `SiNetapp` *to be enabled*.
#[component]
pub fn Netapp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 2v20h9.33V10h5.34v12H24V2Z" /></svg>
   }
}