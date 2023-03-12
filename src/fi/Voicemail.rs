#[cfg(feature = "FiVoicemail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiVoicemail")]
/// *This icon requires the feature* `FiVoicemail` *to be enabled*.
#[component]
pub fn Voicemail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="5.5" cy="11.5" r="4.5" /><circle cx="18.5" cy="11.5" r="4.5" /><line x1="5.5" y1="16" x2="18.5" y2="16" /></svg>
   }
}