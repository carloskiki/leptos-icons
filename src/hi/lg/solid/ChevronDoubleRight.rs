#[cfg(feature = "HiLgSolidChevronDoubleRight")]
use leptos::*;
#[cfg(feature = "HiLgSolidChevronDoubleRight")]
///This icon requires the feature `HiLgSolidChevronDoubleRight` to be enabled.
#[component]
pub fn ChevronDoubleRight(
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
        "M4.71967 3.96967C5.01256 3.67678 5.48744 3.67678 5.78033 3.96967L13.2803 11.4697C13.5732 11.7626 13.5732 12.2374 13.2803 12.5303L5.78033 20.0303C5.48744 20.3232 5.01256 20.3232 4.71967 20.0303C4.42678 19.7374 4.42678 19.2626 4.71967 18.9697L11.6893 12L4.71967 5.03033C4.42678 4.73744 4.42678 4.26256 4.71967 3.96967ZM10.7197 3.96967C11.0126 3.67678 11.4874 3.67678 11.7803 3.96967L19.2803 11.4697C19.5732 11.7626 19.5732 12.2374 19.2803 12.5303L11.7803 20.0303C11.4874 20.3232 11.0126 20.3232 10.7197 20.0303C10.4268 19.7374 10.4268 19.2626 10.7197 18.9697L17.6893 12L10.7197 5.03033C10.4268 4.73744 10.4268 4.26256 10.7197 3.96967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
