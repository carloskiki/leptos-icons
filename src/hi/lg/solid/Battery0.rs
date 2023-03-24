#[cfg(feature = "HiLgSolidBattery0")]
use leptos::*;
#[cfg(feature = "HiLgSolidBattery0")]
///This icon requires the feature `HiLgSolidBattery0` to be enabled.
#[component]
pub fn Battery0(
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
        "M0.75 9.75C0.75 8.09315 2.09315 6.75 3.75 6.75H18.75C20.4069 6.75 21.75 8.09315 21.75 9.75V9.78751C22.6058 9.96123 23.25 10.7179 23.25 11.625V13.875C23.25 14.7821 22.6058 15.5388 21.75 15.7125V15.75C21.75 17.4069 20.4069 18.75 18.75 18.75H3.75C2.09315 18.75 0.75 17.4069 0.75 15.75V9.75ZM20.25 9.75C20.25 8.92157 19.5784 8.25 18.75 8.25H3.75C2.92157 8.25 2.25 8.92157 2.25 9.75V15.75C2.25 16.5784 2.92157 17.25 3.75 17.25H18.75C19.5784 17.25 20.25 16.5784 20.25 15.75V9.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
