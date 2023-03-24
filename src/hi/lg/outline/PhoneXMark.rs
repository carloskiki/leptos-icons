#[cfg(feature = "HiLgOutlinePhoneXMark")]
use leptos::*;
#[cfg(feature = "HiLgOutlinePhoneXMark")]
///This icon requires the feature `HiLgOutlinePhoneXMark` to be enabled.
#[component]
pub fn PhoneXMark(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 3.75L18 6M18 6L20.25 8.25M18 6L20.25 3.75M18 6L15.75 8.25M17.25 21.75C8.96573 21.75 2.25 15.0343 2.25 6.75V4.5C2.25 3.25736 3.25736 2.25 4.5 2.25H5.87163C6.38785 2.25 6.83783 2.60133 6.96304 3.10215L8.06883 7.52533C8.17861 7.96445 8.01453 8.4266 7.65242 8.69818L6.3588 9.6684C5.98336 9.94998 5.81734 10.437 5.97876 10.8777C7.19015 14.1846 9.81539 16.8098 13.1223 18.0212C13.563 18.1827 14.05 18.0166 14.3316 17.6412L15.3018 16.3476C15.5734 15.9855 16.0355 15.8214 16.4747 15.9312L20.8979 17.037C21.3987 17.1622 21.75 17.6121 21.75 18.1284V19.5C21.75 20.7426 20.7426 21.75 19.5 21.75H17.25Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
