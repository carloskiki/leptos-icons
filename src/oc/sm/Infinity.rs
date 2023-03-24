#[cfg(feature = "OcSmInfinity")]
use leptos::*;
#[cfg(feature = "OcSmInfinity")]
///This icon requires the feature `OcSmInfinity` to be enabled.
#[component]
pub fn Infinity(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 6.984c.59-.533 1.204-1.066 1.825-1.493.797-.548 1.7-.991 2.675-.991C14.414 4.5 16 6.086 16 8s-1.586 3.5-3.5 3.5c-.975 0-1.878-.444-2.675-.991-.621-.427-1.235-.96-1.825-1.493-.59.533-1.204 1.066-1.825 1.493-.797.547-1.7.991-2.675.991C1.586 11.5 0 9.914 0 8s1.586-3.5 3.5-3.5c.975 0 1.878.443 2.675.991.621.427 1.235.96 1.825 1.493ZM9.114 8c.536.483 1.052.922 1.56 1.273.704.483 1.3.727 1.826.727 1.086 0 2-.914 2-2 0-1.086-.914-2-2-2-.525 0-1.122.244-1.825.727-.51.35-1.025.79-1.561 1.273ZM3.5 6c-1.086 0-2 .914-2 2 0 1.086.914 2 2 2 .525 0 1.122-.244 1.825-.727.51-.35 1.025-.79 1.561-1.273-.536-.483-1.052-.922-1.56-1.273C4.621 6.244 4.025 6 3.5 6Z"
        /> < title > { title } < / title > < / svg >
    }
}
