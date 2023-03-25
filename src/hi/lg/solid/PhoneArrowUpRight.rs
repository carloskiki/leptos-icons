#[cfg(feature = "HiLgSolidPhoneArrowUpRight")]
use leptos::*;
#[cfg(feature = "HiLgSolidPhoneArrowUpRight")]
///This icon requires the feature `HiLgSolidPhoneArrowUpRight` to be enabled.
#[component]
pub fn PhoneArrowUpRight(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15 3.75C15 3.33579 15.3358 3 15.75 3H20.25C20.6642 3 21 3.33579 21 3.75V8.25C21 8.66421 20.6642 9 20.25 9C19.8358 9 19.5 8.66421 19.5 8.25V5.56066L14.7803 10.2803C14.4874 10.5732 14.0126 10.5732 13.7197 10.2803C13.4268 9.98744 13.4268 9.51256 13.7197 9.21967L18.4393 4.5H15.75C15.3358 4.5 15 4.16421 15 3.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1.5 4.5C1.5 2.84315 2.84315 1.5 4.5 1.5H5.87163C6.732 1.5 7.48197 2.08556 7.69064 2.92025L8.79644 7.34343C8.97941 8.0753 8.70594 8.84555 8.10242 9.29818L6.8088 10.2684C6.67447 10.3691 6.64527 10.5167 6.683 10.6197C7.81851 13.7195 10.2805 16.1815 13.3803 17.317C13.4833 17.3547 13.6309 17.3255 13.7316 17.1912L14.7018 15.8976C15.1545 15.2941 15.9247 15.0206 16.6566 15.2036L21.0798 16.3094C21.9144 16.518 22.5 17.268 22.5 18.1284V19.5C22.5 21.1569 21.1569 22.5 19.5 22.5H17.25C8.55151 22.5 1.5 15.4485 1.5 6.75V4.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
