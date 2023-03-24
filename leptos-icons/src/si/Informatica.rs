#[cfg(feature = "SiInformatica")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiInformatica")]
/// *This icon requires the feature* `SiInformatica` *to be enabled*.
#[component]
pub fn Informatica(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0l3.547 10.788-4.5-1.255-.25 4.43 7.121 4.035V18h.001l5.919-6zm-.64.65L.162 12l6.32 6.407L12 24l5.184-5.255-9.736-3.856z" /></svg>
   }
}