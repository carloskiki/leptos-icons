#[cfg(feature = "TiMediaRecordOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaRecordOutline")]
/// *This icon requires the feature* `TiMediaRecordOutline` *to be enabled*.
#[component]
pub fn MediaRecordOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M12 8c2.205 0 4 1.794 4 4s-1.795 4-4 4-4-1.794-4-4 1.795-4 4-4m0-2c-3.314 0-6 2.686-6 6 0 3.312 2.686 6 6 6 3.312 0 6-2.688 6-6 0-3.314-2.688-6-6-6z" /></svg>
   }
}