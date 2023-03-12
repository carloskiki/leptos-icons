#[cfg(feature = "AiFilledSignal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledSignal")]
/// *This icon requires the feature* `AiFilledSignal` *to be enabled*.
#[component]
pub fn Signal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" t="1569682885975" class="icon" viewBox="0 0 1024 1024" version="1.1" p-id="8305" width="200" height="200"><defs><style type="text/css" /></defs><path d="M584 352H440c-17.7 0-32 14.3-32 32v544c0 17.7 14.3 32 32 32h144c17.7 0 32-14.3 32-32V384c0-17.7-14.3-32-32-32zM892 64H748c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h144c17.7 0 32-14.3 32-32V96c0-17.7-14.3-32-32-32zM276 640H132c-17.7 0-32 14.3-32 32v256c0 17.7 14.3 32 32 32h144c17.7 0 32-14.3 32-32V672c0-17.7-14.3-32-32-32z" p-id="8306" /></svg>
   }
}