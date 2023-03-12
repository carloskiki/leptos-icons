#[cfg(feature = "SiDatocms")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDatocms")]
/// *This icon requires the feature* `SiDatocms` *to be enabled*.
#[component]
pub fn Datocms(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0H.076v24H12c5.964 0 11.924-5.373 11.924-11.998C23.924 5.376 17.963 0 12 0zm0 17.453a5.453 5.453 0 115.455-5.451A5.45 5.45 0 0112 17.452z" /></svg>
   }
}