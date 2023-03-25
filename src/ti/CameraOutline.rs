#[cfg(feature = "TiCameraOutline")]
use leptos::*;
#[cfg(feature = "TiCameraOutline")]
///This icon requires the feature `TiCameraOutline` to be enabled.
#[component]
pub fn CameraOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 20h-14c-1.654 0-3-1.346-3-3v-8c0-1.654 1.346-3 3-3h1.586l1-1c.579-.579 1.596-1 2.414-1h4c.818 0 1.835.421 2.414 1l1 1h1.586c1.654 0 3 1.346 3 3v8c0 1.654-1.346 3-3 3zm-14-12c-.552 0-1 .448-1 1v8c0 .552.448 1 1 1h14c.552 0 1-.448 1-1v-8c0-.552-.448-1-1-1h-2c-.266 0-.52-.105-.707-.293l-1.293-1.293c-.201-.201-.715-.414-1-.414h-4c-.285 0-.799.213-1 .414l-1.293 1.293c-.187.188-.441.293-.707.293h-2zM12 10c1.379 0 2.5 1.121 2.5 2.5s-1.121 2.5-2.5 2.5-2.5-1.121-2.5-2.5 1.121-2.5 2.5-2.5m0-1c-1.934 0-3.5 1.566-3.5 3.5 0 1.932 1.566 3.5 3.5 3.5s3.5-1.568 3.5-3.5c0-1.934-1.566-3.5-3.5-3.5zM18 8.699c-.719 0-1.3.582-1.3 1.301s.581 1.299 1.3 1.299 1.3-.58 1.3-1.299-.581-1.301-1.3-1.301z"
        /> < title > { title } < / title > < / svg >
    }
}
