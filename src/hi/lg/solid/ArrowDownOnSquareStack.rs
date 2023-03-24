#[cfg(feature = "HiLgSolidArrowDownOnSquareStack")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowDownOnSquareStack")]
///This icon requires the feature `HiLgSolidArrowDownOnSquareStack` to be enabled.
#[component]
pub fn ArrowDownOnSquareStack(
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
        "M9.75 6.75H6.75C5.09315 6.75 3.75 8.09315 3.75 9.75V17.25C3.75 18.9069 5.09315 20.25 6.75 20.25H14.25C15.9069 20.25 17.25 18.9069 17.25 17.25V9.75C17.25 8.09315 15.9069 6.75 14.25 6.75H11.25L11.25 1.5C11.25 1.08579 10.9142 0.75 10.5 0.75C10.0858 0.75 9.75 1.08579 9.75 1.5V6.75ZM9.75 6.75H11.25V12.4393L12.9697 10.7197C13.2626 10.4268 13.7374 10.4268 14.0303 10.7197C14.3232 11.0126 14.3232 11.4874 14.0303 11.7803L11.0303 14.7803C10.7374 15.0732 10.2626 15.0732 9.96967 14.7803L6.96967 11.7803C6.67678 11.4874 6.67678 11.0126 6.96967 10.7197C7.26256 10.4268 7.73744 10.4268 8.03033 10.7197L9.75 12.4393V6.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.15137 21.75C7.67008 22.6467 8.6396 23.25 9.75002 23.25H17.25C18.9069 23.25 20.25 21.9069 20.25 20.25V12.75C20.25 11.6396 19.6467 10.6701 18.75 10.1514V17.25C18.75 19.7353 16.7353 21.75 14.25 21.75H7.15137Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
