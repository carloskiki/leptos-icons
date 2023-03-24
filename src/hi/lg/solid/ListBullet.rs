#[cfg(feature = "HiLgSolidListBullet")]
use leptos::*;
#[cfg(feature = "HiLgSolidListBullet")]
///This icon requires the feature `HiLgSolidListBullet` to be enabled.
#[component]
pub fn ListBullet(
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
        "M2.625 6.75C2.625 6.12868 3.12868 5.625 3.75 5.625C4.37132 5.625 4.875 6.12868 4.875 6.75C4.875 7.37132 4.37132 7.875 3.75 7.875C3.12868 7.875 2.625 7.37132 2.625 6.75ZM7.5 6.75C7.5 6.33579 7.83579 6 8.25 6H20.25C20.6642 6 21 6.33579 21 6.75C21 7.16421 20.6642 7.5 20.25 7.5H8.25C7.83579 7.5 7.5 7.16421 7.5 6.75ZM2.625 12C2.625 11.3787 3.12868 10.875 3.75 10.875C4.37132 10.875 4.875 11.3787 4.875 12C4.875 12.6213 4.37132 13.125 3.75 13.125C3.12868 13.125 2.625 12.6213 2.625 12ZM7.5 12C7.5 11.5858 7.83579 11.25 8.25 11.25H20.25C20.6642 11.25 21 11.5858 21 12C21 12.4142 20.6642 12.75 20.25 12.75H8.25C7.83579 12.75 7.5 12.4142 7.5 12ZM2.625 17.25C2.625 16.6287 3.12868 16.125 3.75 16.125C4.37132 16.125 4.875 16.6287 4.875 17.25C4.875 17.8713 4.37132 18.375 3.75 18.375C3.12868 18.375 2.625 17.8713 2.625 17.25ZM7.5 17.25C7.5 16.8358 7.83579 16.5 8.25 16.5H20.25C20.6642 16.5 21 16.8358 21 17.25C21 17.6642 20.6642 18 20.25 18H8.25C7.83579 18 7.5 17.6642 7.5 17.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
