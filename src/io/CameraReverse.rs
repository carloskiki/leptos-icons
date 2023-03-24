#[cfg(feature = "IoCameraReverse")]
use leptos::*;
#[cfg(feature = "IoCameraReverse")]
///This icon requires the feature `IoCameraReverse` to be enabled.
#[component]
pub fn CameraReverse(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,144H373c-3,0-6.72-1.94-9.62-5L337.44,98.06a15.52,15.52,0,0,0-1.37-1.85C327.11,85.76,315,80,302,80H210c-13,0-25.11,5.76-34.07,16.21a15.52,15.52,0,0,0-1.37,1.85l-25.94,41c-2.22,2.42-5.34,5-8.62,5v-8a16,16,0,0,0-16-16H100a16,16,0,0,0-16,16v8H80a48.05,48.05,0,0,0-48,48V384a48.05,48.05,0,0,0,48,48H432a48.05,48.05,0,0,0,48-48V192A48.05,48.05,0,0,0,432,144ZM316.84,346.3a96.06,96.06,0,0,1-155.66-59.18,16,16,0,0,1-16.49-26.43l20-20a16,16,0,0,1,22.62,0l20,20A16,16,0,0,1,196,288a17.31,17.31,0,0,1-2-.14,64.07,64.07,0,0,0,102.66,33.63,16,16,0,1,1,20.21,24.81Zm50.47-63-20,20a16,16,0,0,1-22.62,0l-20-20a16,16,0,0,1,13.09-27.2A64,64,0,0,0,215,222.64,16,16,0,1,1,194.61,198a96,96,0,0,1,156,59,16,16,0,0,1,16.72,26.35Z"
        /> < title > { title } < / title > < / svg >
    }
}
