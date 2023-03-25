#[cfg(feature = "OcSmBeaker")]
use leptos::*;
#[cfg(feature = "OcSmBeaker")]
///This icon requires the feature `OcSmBeaker` to be enabled.
#[component]
pub fn Beaker(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5 5.782V2.5h-.25a.75.75 0 0 1 0-1.5h6.5a.75.75 0 0 1 0 1.5H11v3.282l3.666 5.76C15.619 13.04 14.543 15 12.767 15H3.233c-1.776 0-2.852-1.96-1.899-3.458Zm-2.4 6.565a.75.75 0 0 0 .633 1.153h9.534a.75.75 0 0 0 .633-1.153L12.225 10.5h-8.45ZM9.5 2.5h-3V6c0 .143-.04.283-.117.403L4.73 9h6.54L9.617 6.403A.746.746 0 0 1 9.5 6Z"
        /> < title > { title } < / title > < / svg >
    }
}
