#[cfg(feature = "HiMdSolidArrowDownOnSquare")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowDownOnSquare")]
///This icon requires the feature `HiMdSolidArrowDownOnSquare` to be enabled.
#[component]
pub fn ArrowDownOnSquare(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.75 7H10.75V12.2955L12.6925 10.2483C12.9696 9.94039 13.4438 9.91544 13.7517 10.1925C14.0596 10.4696 14.0846 10.9438 13.8075 11.2517L10.5575 14.7517C10.4152 14.9098 10.2126 15 10 15C9.78739 15 9.58477 14.9098 9.44254 14.7517L6.19254 11.2517C5.91544 10.9438 5.9404 10.4696 6.24828 10.1925C6.55617 9.91544 7.03038 9.94039 7.30748 10.2483L9.25001 12.2955V7H10.75L10.75 1.75C10.75 1.33579 10.4142 1 10 1C9.58579 1 9.25 1.33579 9.25 1.75L9.25001 7H6.25C5.00736 7 4 8.00736 4 9.25V16.75C4 17.9926 5.00736 19 6.25 19H13.75C14.9926 19 16 17.9926 16 16.75V9.25C16 8.00736 14.9926 7 13.75 7Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
