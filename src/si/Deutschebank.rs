#[cfg(feature = "SiDeutschebank")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDeutschebank")]
/// *This icon requires the feature* `SiDeutschebank` *to be enabled*.
#[component]
pub fn Deutschebank(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M3.375 3.375v17.25h17.25V3.375H3.375zM0 0h24v24H0V0zm5.25 18.225 9.15-12.45h4.35L9.6 18.225H5.25z" /></svg>
   }
}