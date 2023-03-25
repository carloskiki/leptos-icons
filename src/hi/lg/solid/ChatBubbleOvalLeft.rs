#[cfg(feature = "HiLgSolidChatBubbleOvalLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleOvalLeft")]
///This icon requires the feature `HiLgSolidChatBubbleOvalLeft` to be enabled.
#[component]
pub fn ChatBubbleOvalLeft(
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
        "M5.33691 21.7178C5.15732 21.7003 4.9793 21.6757 4.80365 21.6442C4.5396 21.597 4.3209 21.4123 4.23008 21.1599C4.13927 20.9075 4.19016 20.6258 4.36357 20.4211C4.76687 19.9451 5.05254 19.3685 5.17822 18.7349C5.20107 18.6196 5.15571 18.4182 4.92371 18.1923C3.2746 16.5871 2.25 14.4086 2.25 12C2.25 6.96934 6.67799 3 12 3C17.322 3 21.75 6.96934 21.75 12C21.75 17.0307 17.322 21 12 21C11.1668 21 10.3569 20.9034 9.58317 20.7213C8.54447 21.3731 7.3153 21.75 6 21.75C5.77647 21.75 5.55516 21.7391 5.33691 21.7178Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
