#[cfg(feature = "FaSolidBlenderPhone")]
use leptos::*;
#[cfg(feature = "FaSolidBlenderPhone")]
///This icon requires the feature `FaSolidBlenderPhone` to be enabled.
#[component]
pub fn BlenderPhone(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M224 352L196.8 52.3C194.2 24.2 216.3 0 244.6 0H534.1c21.1 0 36.4 20.1 30.9 40.4L558.5 64H400c-8.8 0-16 7.2-16 16s7.2 16 16 16H549.8l-17.5 64H400c-8.8 0-16 7.2-16 16s7.2 16 16 16H523.6l-17.5 64H400c-8.8 0-16 7.2-16 16s7.2 16 16 16h97.5L480 352H224zm-16 32H496c26.5 0 48 21.5 48 48v32c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V432c0-26.5 21.5-48 48-48zm144 96a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM147.5 30.7c10.8 6.7 15.3 21 10.6 33.4l-22 57.8c-4.2 10.9-14.5 17.6-25.3 16.4l-33.3-3.6c-13.6 42.2-13.6 88.4 0 130.7l33.3-3.6c10.9-1.2 21.2 5.5 25.3 16.4l22 57.8c4.7 12.4 .2 26.7-10.6 33.4l-44 27.2c-9.7 6-21.9 4.2-29.8-4.3C-24.6 286-24.6 114 73.7 7.8C81.6-.7 93.8-2.5 103.5 3.5l44 27.2z"
        /> < title > { title } < / title > < / svg >
    }
}
