#[cfg(feature = "HiLgOutlineSquaresPlus")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSquaresPlus")]
///This icon requires the feature `HiLgOutlineSquaresPlus` to be enabled.
#[component]
pub fn SquaresPlus(
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
        "M13.5 16.875H16.875M16.875 16.875H20.25M16.875 16.875V13.5M16.875 16.875V20.25M6 10.5H8.25C9.49264 10.5 10.5 9.49264 10.5 8.25V6C10.5 4.75736 9.49264 3.75 8.25 3.75H6C4.75736 3.75 3.75 4.75736 3.75 6V8.25C3.75 9.49264 4.75736 10.5 6 10.5ZM6 20.25H8.25C9.49264 20.25 10.5 19.2426 10.5 18V15.75C10.5 14.5074 9.49264 13.5 8.25 13.5H6C4.75736 13.5 3.75 14.5074 3.75 15.75V18C3.75 19.2426 4.75736 20.25 6 20.25ZM15.75 10.5H18C19.2426 10.5 20.25 9.49264 20.25 8.25V6C20.25 4.75736 19.2426 3.75 18 3.75H15.75C14.5074 3.75 13.5 4.75736 13.5 6V8.25C13.5 9.49264 14.5074 10.5 15.75 10.5Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
