#[cfg(feature = "SiScreencastify")]
use leptos::*;
#[cfg(feature = "SiScreencastify")]
///This icon requires the feature `SiScreencastify` to be enabled.
#[component]
pub fn Screencastify(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.898 2.347c-.472.008-.914.38-.914.891v4.278H1.1c-.541 0-1.1.437-1.1.978v7.02c0 .54.559.907 1.1.907h5.884V7.533h6.408c.542 0 .926.437.926.979v1.623l3.667-2.095v7.927l-3.667-2.095v1.676c0 .541-.384.908-.926.908H6.984v4.313c0 .68.786 1.1 1.38.768l9.638-5.535 5.553-3.195c.593-.402.593-1.257 0-1.59l-5.553-3.194L8.364 2.47a.886.886 0 00-.466-.123z"
        /> < title > { title } < / title > < / svg >
    }
}
