#[cfg(feature = "SiTidal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTidal")]
/// *This icon requires the feature* `SiTidal` *to be enabled*.
#[component]
pub fn Tidal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12.012 3.992L8.008 7.996 4.004 3.992 0 7.996 4.004 12l4.004-4.004L12.012 12l-4.004 4.004 4.004 4.004 4.004-4.004L12.012 12l4.004-4.004-4.004-4.004zM16.042 7.996l3.979-3.979L24 7.996l-3.979 3.979z" /></svg>
   }
}