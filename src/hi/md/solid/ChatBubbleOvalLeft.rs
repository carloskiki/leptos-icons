#[cfg(feature = "HiMdSolidChatBubbleOvalLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleOvalLeft")]
///This icon requires the feature `HiMdSolidChatBubbleOvalLeft` to be enabled.
#[component]
pub fn ChatBubbleOvalLeft(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 10C2 6.0334 5.69006 3 10 3C14.3099 3 18 6.0334 18 10C18 13.9666 14.3099 17 10 17C9.48658 17 8.98381 16.9577 8.49617 16.8766C7.51177 17.5834 6.3037 18 5 18C4.85237 18 4.70585 17.9947 4.56065 17.9841C4.29888 17.9651 4.06608 17.8107 3.9468 17.5769C3.82753 17.3431 3.83914 17.064 3.97742 16.8409C4.29476 16.329 4.48371 15.7294 4.49899 15.0848C2.97849 13.8253 2 12.0244 2 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
