#[cfg(feature = "CgCornerRightUp")]
use leptos::*;
#[cfg(feature = "CgCornerRightUp")]
///This icon requires the feature `CgCornerRightUp` to be enabled.
#[component]
pub fn CornerRightUp(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.28999 10.6254L7.87709 9.20989L14.2469 2.85181L20.605 9.22164L19.1895 10.6346L15.4082 6.84631L15.3987 17.152C15.3967 19.3611 13.6042 21.1503 11.395 21.1483L3.39502 21.1409L3.39687 19.1409L11.3969 19.1483C12.5014 19.1493 13.3977 18.2547 13.3987 17.1501L13.4085 6.51446L9.28999 10.6254Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
