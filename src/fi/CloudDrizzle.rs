#[cfg(feature = "FiCloudDrizzle")]
use leptos::*;
#[cfg(feature = "FiCloudDrizzle")]
///This icon requires the feature `FiCloudDrizzle` to be enabled.
#[component]
pub fn CloudDrizzle(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "8" y1 = "19" x2 = "8" y2 = "21" />< line xmlns = "http://www.w3.org/2000/svg" x1
        = "8" y1 = "13" x2 = "8" y2 = "15" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "16" y1 = "19" x2 = "16" y2 = "21" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "16" y1 = "13" x2 = "16" y2 = "15" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "21" x2 = "12" y2 = "23" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "15" x2 = "12" y2 = "17"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" /> < title > { title } < /
        title > < / svg >
    }
}
