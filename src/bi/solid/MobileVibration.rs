#[cfg(feature = "BiSolidMobileVibration")]
use leptos::*;
#[cfg(feature = "BiSolidMobileVibration")]
///This icon requires the feature `BiSolidMobileVibration` to be enabled.
#[component]
pub fn MobileVibration(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15.535 2.808a2.003 2.003 0 0 0-2.828 0l-9.899 9.899a2.001 2.001 0 0 0 0 2.828l5.657 5.657c.39.39.902.585 1.414.585s1.024-.195 1.414-.585l9.899-9.899c.78-.779.78-2.049 0-2.828l-5.657-5.657zM8.707 16.707a.999.999 0 1 1-1.414-1.414.999.999 0 1 1 1.414 1.414zm7 5-1.414-1.414 6-6 1.414 1.415zM8.293 2.293l1.414 1.414-6 6-1.414-1.415z"
        /> < title > { title } < / title > < / svg >
    }
}
