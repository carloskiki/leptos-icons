#[cfg(feature = "HiMdSolidCreditCard")]
use leptos::*;
#[cfg(feature = "HiMdSolidCreditCard")]
///This icon requires the feature `HiMdSolidCreditCard` to be enabled.
#[component]
pub fn CreditCard(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2.5 4C1.67157 4 1 4.67157 1 5.5V6H19V5.5C19 4.67157 18.3284 4 17.5 4H2.5ZM19 8.5H1V14.5C1 15.3284 1.67157 16 2.5 16H17.5C18.3284 16 19 15.3284 19 14.5V8.5ZM3 13.25C3 12.8358 3.33579 12.5 3.75 12.5H5.25C5.66421 12.5 6 12.8358 6 13.25C6 13.6642 5.66421 14 5.25 14H3.75C3.33579 14 3 13.6642 3 13.25ZM7.75 12.5C7.33579 12.5 7 12.8358 7 13.25C7 13.6642 7.33579 14 7.75 14H11.25C11.6642 14 12 13.6642 12 13.25C12 12.8358 11.6642 12.5 11.25 12.5H7.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
