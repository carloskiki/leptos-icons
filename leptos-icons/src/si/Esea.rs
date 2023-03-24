#[cfg(feature = "SiEsea")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiEsea")]
/// *This icon requires the feature* `SiEsea` *to be enabled*.
#[component]
pub fn Esea(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M14.054 2.767L8.95 9.511 0 9.65l5.832 4.47L1.042 21l8.491-4.088 5.711 4.322V14.12L24 9.796l-17.255 4.02a12.575 12.575 0 001.589-1.955 5.475 5.475 0 00.617-1.786l5.593-.15z" /></svg>
   }
}