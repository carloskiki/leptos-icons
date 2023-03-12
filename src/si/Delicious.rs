#[cfg(feature = "SiDelicious")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDelicious")]
/// *This icon requires the feature* `SiDelicious` *to be enabled*.
#[component]
pub fn Delicious(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 12H0v12h12V12zM24 0H12v12h12V0z" /></svg>
   }
}