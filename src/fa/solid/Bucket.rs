#[cfg(feature = "FaSolidBucket")]
use leptos::*;
#[cfg(feature = "FaSolidBucket")]
///This icon requires the feature `FaSolidBucket` to be enabled.
#[component]
pub fn Bucket(
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
        "M96 152v8H48v-8C48 68.1 116.1 0 200 0h48c83.9 0 152 68.1 152 152v8H352v-8c0-57.4-46.6-104-104-104H200C142.6 48 96 94.6 96 152zM0 224c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32h-5.1L388.5 469c-2.6 24.4-23.2 43-47.7 43H107.2c-24.6 0-45.2-18.5-47.7-43L37.1 256H32c-17.7 0-32-14.3-32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
