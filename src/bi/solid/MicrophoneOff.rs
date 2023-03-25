#[cfg(feature = "BiSolidMicrophoneOff")]
use leptos::*;
#[cfg(feature = "BiSolidMicrophoneOff")]
///This icon requires the feature `BiSolidMicrophoneOff` to be enabled.
#[component]
pub fn MicrophoneOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m21.707 20.293-3.4-3.4A7.93 7.93 0 0 0 20 12h-2a5.945 5.945 0 0 1-1.119 3.467l-1.449-1.45A3.926 3.926 0 0 0 16 12V6c0-2.217-1.785-4.021-3.979-4.021-.07 0-.14.009-.209.025A4.006 4.006 0 0 0 8 6v.586L3.707 2.293 2.293 3.707l18 18 1.414-1.414zM6 12H4c0 4.072 3.06 7.436 7 7.931V22h2v-2.069a7.935 7.935 0 0 0 2.241-.63l-1.549-1.548A5.983 5.983 0 0 1 12 18c-3.309 0-6-2.691-6-6z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.007 12.067a3.996 3.996 0 0 0 3.926 3.926l-3.926-3.926z" /> < title > { title
        } < / title > < / svg >
    }
}
