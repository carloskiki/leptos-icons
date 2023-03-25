#[cfg(feature = "BiRegularCoin")]
use leptos::*;
#[cfg(feature = "BiRegularCoin")]
///This icon requires the feature `BiRegularCoin` to be enabled.
#[component]
pub fn Coin(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 6C7.03 6 2 7.546 2 10.5v4C2 17.454 7.03 19 12 19s10-1.546 10-4.5v-4C22 7.546 16.97 6 12 6zm-8 8.5v-1.197a9.989 9.989 0 0 0 2 .86v1.881c-1.312-.514-2-1.126-2-1.544zm12 .148v1.971c-.867.179-1.867.31-3 .358v-2a21.75 21.75 0 0 0 3-.329zm-5 2.33a18.788 18.788 0 0 1-3-.358v-1.971c.959.174 1.972.287 3 .33v1.999zm7-.934v-1.881a9.931 9.931 0 0 0 2-.86V14.5c0 .418-.687 1.03-2 1.544zM12 13c-5.177 0-8-1.651-8-2.5S6.823 8 12 8s8 1.651 8 2.5-2.823 2.5-8 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
