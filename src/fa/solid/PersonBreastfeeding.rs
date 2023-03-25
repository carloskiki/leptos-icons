#[cfg(feature = "FaSolidPersonBreastfeeding")]
use leptos::*;
#[cfg(feature = "FaSolidPersonBreastfeeding")]
///This icon requires the feature `FaSolidPersonBreastfeeding` to be enabled.
#[component]
pub fn PersonBreastfeeding(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M224 0a80 80 0 1 1 0 160A80 80 0 1 1 224 0zM436.8 382.8L373.5 462c-16.6 20.7-46.8 24.1-67.5 7.5c-17.6-14.1-22.7-38.1-13.5-57.7l-.7-.1c-38.9-5.6-74.3-25.1-99.7-54.8V320c0-17.7-14.3-32-32-32s-32 14.3-32 32v48c0 .8 0 1.6 .1 2.4l101.4 50.7c23.7 11.9 33.3 40.7 21.5 64.4s-40.7 33.3-64.4 21.5L27.2 427.3c-1.1-.5-2.2-1.1-3.3-1.7c-4.9-2.8-9.2-6.4-12.6-10.6c-4.6-5.4-7.8-11.7-9.6-18.4c-3.3-12-1.9-25.2 4.8-36.6c.6-1.1 1.3-2.2 2-3.2L75.6 256.1c26.7-40.1 71.7-64.1 119.8-64.1h75.2c46.5 0 90.1 22.5 117.2 60.3l50.7 70.9c2.2 3 4 6.1 5.5 9.4c2.9 6.7 4.3 13.8 4 20.8c-.3 10.6-4.2 21-11.2 29.4zM320 332a44 44 0 1 0 -88 0 44 44 0 1 0 88 0z"
        /> < title > { title } < / title > < / svg >
    }
}
