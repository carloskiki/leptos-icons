#[cfg(feature = "HiLgSolidChatBubbleBottomCenter")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleBottomCenter")]
///This icon requires the feature `HiLgSolidChatBubbleBottomCenter` to be enabled.
#[component]
pub fn ChatBubbleBottomCenter(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M4.84836 2.771C7.18302 2.42773 9.57113 2.25 12.0003 2.25C14.4292 2.25 16.8171 2.4277 19.1516 2.77091C21.1299 3.06177 22.5 4.79445 22.5 6.74056V12.7595C22.5 14.7056 21.1299 16.4382 19.1516 16.7291C18.0054 16.8976 16.8464 17.0262 15.6759 17.1135C15.5514 17.1228 15.4431 17.1875 15.379 17.2835L12.624 21.416C12.4849 21.6247 12.2508 21.75 12 21.75C11.7492 21.75 11.5151 21.6247 11.376 21.416L8.62092 17.2835C8.5569 17.1874 8.44853 17.1227 8.32408 17.1134C7.15358 17.0261 5.99452 16.8975 4.84836 16.729C2.87004 16.4381 1.5 14.7054 1.5 12.7593V6.74064C1.5 4.79455 2.87004 3.06188 4.84836 2.771Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
