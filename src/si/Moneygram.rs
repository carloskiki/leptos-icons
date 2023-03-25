#[cfg(feature = "SiMoneygram")]
use leptos::*;
#[cfg(feature = "SiMoneygram")]
///This icon requires the feature `SiMoneygram` to be enabled.
#[component]
pub fn Moneygram(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24 12c0 6.6274-5.3726 12-12 12S0 18.6274 0 12c0-1.8257.4071-3.5554 1.1374-5.1051C.6514 8.1257.433 9.3446.433 10.4863c0 5.4334 4.3868 6.2203 6.2537 6.2023 2.8371-.0257 6.1543-1.416 8.9485-3.9909l-.4714 2.6494c-.1054.606.2906 1.1392.8957 1.1426h.2503c.6274 0 1.0732-.5108 1.1863-1.1426l1.0063-5.6622c.12-.6283-.2932-1.14-.9214-1.14h-5.6726c-.6309 0-1.2077.3342-1.32.9677l-.0446.2554c-.09.6026.33 1.0569.9317 1.0569h2.9589a9.48 9.48 0 0 0-.1414.1388c-2.04 1.9312-4.5558 2.988-6.6403 2.988-2.0803 0-4.41-1.3123-4.41-4.2686C3.2426 3.5546 8.9906 0 12 0c6.6137 0 12 5.3726 12 12"
        /> < title > { title } < / title > < / svg >
    }
}
