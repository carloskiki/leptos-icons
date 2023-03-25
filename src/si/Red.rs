#[cfg(feature = "SiRed")]
use leptos::*;
#[cfg(feature = "SiRed")]
///This icon requires the feature `SiRed` to be enabled.
#[component]
pub fn Red(
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
        "M12 6.679V0L8.655 4.945Zm0 1.976v6.69l7.673-4L16.327 6.4zm0-1.976 3.345-1.734L12 0Zm8.655 6.133L12 17.322V24l12-6.242ZM12 24v-6.679l-8.655-4.509L0 17.758ZM4.327 11.345l7.673 4v-6.69L7.673 6.4Z"
        /> < title > { title } < / title > < / svg >
    }
}
