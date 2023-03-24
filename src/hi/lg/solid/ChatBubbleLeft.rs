#[cfg(feature = "HiLgSolidChatBubbleLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleLeft")]
///This icon requires the feature `HiLgSolidChatBubbleLeft` to be enabled.
#[component]
pub fn ChatBubbleLeft(
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
        "M4.84836 2.771C7.18302 2.42773 9.57113 2.25 12.0003 2.25C14.4292 2.25 16.8171 2.4277 19.1516 2.77091C21.1299 3.06177 22.5 4.79445 22.5 6.74056V12.7595C22.5 14.7056 21.1299 16.4382 19.1516 16.7291C17.2123 17.0142 15.2361 17.1851 13.2302 17.2348C13.1266 17.2374 13.0318 17.2788 12.9638 17.3468L8.78033 21.5303C8.56583 21.7448 8.24324 21.809 7.96299 21.6929C7.68273 21.5768 7.5 21.3033 7.5 21V17.045C6.60901 16.9634 5.72491 16.8579 4.84836 16.729C2.87004 16.4381 1.5 14.7054 1.5 12.7593V6.74064C1.5 4.79455 2.87004 3.06188 4.84836 2.771Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
