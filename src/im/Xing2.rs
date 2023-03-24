#[cfg(feature = "ImXing2")]
use leptos::*;
#[cfg(feature = "ImXing2")]
///This icon requires the feature `ImXing2` to be enabled.
#[component]
pub fn Xing2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M2.431 3.159c-0.138 0-0.256 0.050-0.316 0.144-0.059 0.1-0.050 0.225 0.013 0.353l1.559 2.7c0.003 0.006 0.003 0.009 0 0.013l-2.45 4.331c-0.063 0.128-0.059 0.256 0 0.353 0.059 0.094 0.163 0.156 0.3 0.156h2.306c0.344 0 0.513-0.234 0.628-0.447 0 0 2.397-4.241 2.491-4.406-0.009-0.016-1.588-2.766-1.588-2.766-0.116-0.203-0.287-0.431-0.644-0.431h-2.3z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.125 0c-0.344 0-0.494 0.216-0.619 0.441 0 0-4.972 8.816-5.134 9.106 0.009 0.016 3.278 6.016 3.278 6.016 0.116 0.203 0.291 0.441 0.644 0.441h2.306c0.137 0 0.247-0.053 0.306-0.147 0.063-0.1 0.059-0.228-0.006-0.356l-3.25-5.947c-0.003-0.006-0.003-0.009 0-0.016l5.109-9.034c0.063-0.128 0.066-0.256 0.006-0.356-0.059-0.094-0.169-0.147-0.306-0.147h-2.334z"
        /> < title > { title } < / title > < / svg >
    }
}
