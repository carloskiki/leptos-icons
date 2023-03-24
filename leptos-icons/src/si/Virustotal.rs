#[cfg(feature = "SiVirustotal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVirustotal")]
/// *This icon requires the feature* `SiVirustotal` *to be enabled*.
#[component]
pub fn Virustotal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M10.87 12L0 22.68h24V1.32H0zm10.73 8.52H5.28l8.637-8.448L5.28 3.48H21.6z" /></svg>
   }
}