#[cfg(feature = "HiLgSolidPaintBrush")]
use leptos::*;
#[cfg(feature = "HiLgSolidPaintBrush")]
///This icon requires the feature `HiLgSolidPaintBrush` to be enabled.
#[component]
pub fn PaintBrush(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M20.5986 1.5C20.2232 1.5 19.8562 1.61111 19.5439 1.81934L14.4649 5.20533C13.1853 6.05835 12.0203 7.0624 10.993 8.19218C13.1064 9.18391 14.8161 10.8935 15.8078 13.007C16.9376 11.9797 17.9416 10.8146 18.7946 9.53508L22.1806 4.45609C22.3888 4.14375 22.5 3.77677 22.5 3.40139C22.5 2.35128 21.6487 1.5 20.5986 1.5ZM12.2995 15.5249C12.9568 15.1597 13.5898 14.7563 14.1954 14.3175C13.3836 12.258 11.742 10.6164 9.68246 9.80456C9.24361 10.4102 8.84023 11.0432 8.47506 11.7005L8.19653 12.2018C9.93302 12.6985 11.3015 14.0669 11.7981 15.8034L12.2995 15.5249ZM6.74995 13.5C4.67886 13.5 2.99995 15.1789 2.99995 17.25C2.99995 18.0784 2.32839 18.75 1.49995 18.75C1.46599 18.75 1.43219 18.7489 1.3986 18.7466C1.12245 18.7284 0.858681 18.8637 0.712418 19.0986C0.566154 19.3336 0.561166 19.63 0.699441 19.8697C1.60524 21.4402 3.30337 22.5 5.24995 22.5C8.14946 22.5 10.5 20.1495 10.5 17.25C10.5 15.1789 8.82104 13.5 6.74995 13.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
