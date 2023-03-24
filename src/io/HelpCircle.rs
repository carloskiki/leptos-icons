#[cfg(feature = "IoHelpCircle")]
use leptos::*;
#[cfg(feature = "IoHelpCircle")]
///This icon requires the feature `IoHelpCircle` to be enabled.
#[component]
pub fn HelpCircle(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Zm-6,304a20,20,0,1,1,20-20A20,20,0,0,1,250,368Zm33.44-102C267.23,276.88,265,286.85,265,296a14,14,0,0,1-28,0c0-21.91,10.08-39.33,30.82-53.26C287.1,229.8,298,221.6,298,203.57c0-12.26-7-21.57-21.49-28.46-3.41-1.62-11-3.2-20.34-3.09-11.72.15-20.82,2.95-27.83,8.59C215.12,191.25,214,202.83,214,203a14,14,0,1,1-28-1.35c.11-2.43,1.8-24.32,24.77-42.8,11.91-9.58,27.06-14.56,45-14.78,12.7-.15,24.63,2,32.72,5.82C312.7,161.34,326,180.43,326,203.57,326,237.4,303.39,252.59,283.44,266Z"
        /> < title > { title } < / title > < / svg >
    }
}
