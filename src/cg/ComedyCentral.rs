#[cfg(feature = "CgComedyCentral")]
use leptos::*;
#[cfg(feature = "CgComedyCentral")]
///This icon requires the feature `CgComedyCentral` to be enabled.
#[component]
pub fn ComedyCentral(
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
        "M10.5445 19C14.4105 19 17.5445 15.866 17.5445 12C17.5445 8.13401 14.4105 5 10.5445 5C8.61148 5 6.86148 5.7835 5.59473 7.05025L3.47343 4.92896L3.45544 4.94694C5.26649 3.12672 7.7739 2 10.5445 2C16.0673 2 20.5445 6.47715 20.5445 12C20.5445 17.5228 16.0673 22 10.5445 22C7.78307 22 5.28308 20.8807 3.47343 19.0711L5.59475 16.9498C6.8615 18.2165 8.61149 19 10.5445 19Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5445 14C11.1376 14 11.6704 13.7418 12.0367 13.3317L14.1594 15.4544C13.249 16.4068 11.966 17 10.5445 17C7.78305 17 5.54447 14.7614 5.54447 12C5.54447 9.23858 7.78305 7 10.5445 7C11.966 7 13.249 7.59323 14.1594 8.54563L12.0367 10.6683C11.6704 10.2582 11.1376 10 10.5445 10C9.4399 10 8.54447 10.8954 8.54447 12C8.54447 13.1046 9.4399 14 10.5445 14Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
