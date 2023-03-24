#[cfg(feature = "SiInstructure")]
use leptos::*;
#[cfg(feature = "SiInstructure")]
///This icon requires the feature `SiInstructure` to be enabled.
#[component]
pub fn Instructure(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m11.996 0-5.11 2.878L12 5.76l5.115-2.878ZM6.032 3.36.918 6.237 6.036 9.12l5.115-2.879Zm11.929 0-5.112 2.878 5.115 2.882 5.118-2.879zM12 11.52.918 17.76 12 24l11.082-6.241Z"
        /> < title > { title } < / title > < / svg >
    }
}
