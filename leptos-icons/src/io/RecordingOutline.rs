#[cfg(feature = "IoRecordingOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRecordingOutline")]
/// *This icon requires the feature* `IoRecordingOutline` *to be enabled*.
#[component]
pub fn RecordingOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="128" cy="256" r="96" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="384" cy="256" r="96" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="128" y1="352" x2="384" y2="352" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}