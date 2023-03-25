#[cfg(feature = "HiMdSolidSquaresPlus")]
use leptos::*;
#[cfg(feature = "HiMdSolidSquaresPlus")]
///This icon requires the feature `HiMdSolidSquaresPlus` to be enabled.
#[component]
pub fn SquaresPlus(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 4.25C2 3.00736 3.00736 2 4.25 2H6.75C7.99264 2 9 3.00736 9 4.25V6.75C9 7.99264 7.99264 9 6.75 9H4.25C3.00736 9 2 7.99264 2 6.75V4.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 13.25C2 12.0074 3.00736 11 4.25 11H6.75C7.99264 11 9 12.0074 9 13.25V15.75C9 16.9926 7.99264 18 6.75 18H4.25C3.00736 18 2 16.9926 2 15.75V13.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 4.25C11 3.00736 12.0074 2 13.25 2H15.75C16.9926 2 18 3.00736 18 4.25V6.75C18 7.99264 16.9926 9 15.75 9H13.25C12.0074 9 11 7.99264 11 6.75V4.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.25 11.75C15.25 11.3358 14.9142 11 14.5 11C14.0858 11 13.75 11.3358 13.75 11.75V13.75H11.75C11.3358 13.75 11 14.0858 11 14.5C11 14.9142 11.3358 15.25 11.75 15.25H13.75V17.25C13.75 17.6642 14.0858 18 14.5 18C14.9142 18 15.25 17.6642 15.25 17.25V15.25H17.25C17.6642 15.25 18 14.9142 18 14.5C18 14.0858 17.6642 13.75 17.25 13.75H15.25V11.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
