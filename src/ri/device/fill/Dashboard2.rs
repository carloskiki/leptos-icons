#[cfg(feature = "RiDeviceFillDashboard2")]
use leptos::*;
#[cfg(feature = "RiDeviceFillDashboard2")]
///This icon requires the feature `RiDeviceFillDashboard2` to be enabled.
#[component]
pub fn Dashboard2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2zm0 3c-3.866 0-7 3.134-7 7 0 1.852.72 3.537 1.894 4.789l.156.16 1.414-1.413C7.56 14.63 7 13.38 7 12c0-2.761 2.239-5 5-5 .448 0 .882.059 1.295.17l1.563-1.562C13.985 5.218 13.018 5 12 5zm6.392 4.143l-1.561 1.562c.11.413.169.847.169 1.295 0 1.38-.56 2.63-1.464 3.536l1.414 1.414C18.216 15.683 19 13.933 19 12c0-1.018-.217-1.985-.608-2.857zm-2.15-2.8l-3.725 3.724C12.352 10.023 12.179 10 12 10c-1.105 0-2 .895-2 2s.895 2 2 2 2-.895 2-2c0-.179-.023-.352-.067-.517l3.724-3.726-1.414-1.414z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
