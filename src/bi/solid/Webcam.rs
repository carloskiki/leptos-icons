#[cfg(feature = "BiSolidWebcam")]
use leptos::*;
#[cfg(feature = "BiSolidWebcam")]
///This icon requires the feature `BiSolidWebcam` to be enabled.
#[component]
pub fn Webcam(
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
        "M12 2c-4.963 0-9 4.037-9 9 0 3.328 1.82 6.232 4.513 7.79l-2.067 1.378A1 1 0 0 0 6 22h12a1 1 0 0 0 .555-1.832l-2.067-1.378C19.18 17.232 21 14.328 21 11c0-4.963-4.037-9-9-9zm0 16c-3.859 0-7-3.141-7-7s3.141-7 7-7 7 3.141 7 7-3.141 7-7 7z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 6c-2.757 0-5 2.243-5 5s2.243 5 5 5 5-2.243 5-5-2.243-5-5-5zm-1.5 5a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 10.5 11z"
        /> < title > { title } < / title > < / svg >
    }
}
