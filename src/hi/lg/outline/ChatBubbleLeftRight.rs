#[cfg(feature = "HiLgOutlineChatBubbleLeftRight")]
use leptos::*;
#[cfg(feature = "HiLgOutlineChatBubbleLeftRight")]
///This icon requires the feature `HiLgOutlineChatBubbleLeftRight` to be enabled.
#[component]
pub fn ChatBubbleLeftRight(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.25 8.51104C21.1341 8.79549 21.75 9.6392 21.75 10.6082V14.8938C21.75 16.0304 20.9026 16.9943 19.7697 17.0867C19.4308 17.1144 19.0909 17.1386 18.75 17.1592V20.25L15.75 17.25C14.3963 17.25 13.0556 17.1948 11.7302 17.0866C11.4319 17.0623 11.1534 16.9775 10.9049 16.8451M20.25 8.51104C20.0986 8.46232 19.9393 8.43 19.7739 8.41628C18.4472 8.30616 17.1051 8.25 15.75 8.25C14.3948 8.25 13.0528 8.30616 11.7261 8.41627C10.595 8.51015 9.75 9.47323 9.75 10.6082V14.8937C9.75 15.731 10.2099 16.4746 10.9049 16.8451M20.25 8.51104V6.63731C20.25 5.01589 19.0983 3.61065 17.4903 3.40191C15.4478 3.13676 13.365 3 11.2503 3C9.13533 3 7.05233 3.13678 5.00963 3.40199C3.40173 3.61074 2.25 5.01598 2.25 6.63738V12.8626C2.25 14.484 3.40173 15.8893 5.00964 16.098C5.58661 16.1729 6.16679 16.2376 6.75 16.2918V21L10.9049 16.8451"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
