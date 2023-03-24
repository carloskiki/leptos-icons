#[cfg(feature = "HiMdSolidBookmarkSquare")]
use leptos::*;
#[cfg(feature = "HiMdSolidBookmarkSquare")]
///This icon requires the feature `HiMdSolidBookmarkSquare` to be enabled.
#[component]
pub fn BookmarkSquare(
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
        "M4.25 2C3.00736 2 2 3.00736 2 4.25V15.75C2 16.9926 3.00736 18 4.25 18H15.75C16.9926 18 18 16.9926 18 15.75V4.25C18 3.00736 16.9926 2 15.75 2H4.25ZM6 13.25V3.5H14V13.25C14 13.5058 13.8697 13.7439 13.6542 13.8818C13.4388 14.0196 13.1679 14.0382 12.9357 13.931L10 12.576L7.06429 13.931C6.83207 14.0382 6.56123 14.0196 6.34578 13.8818C6.13034 13.7439 6 13.5058 6 13.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
