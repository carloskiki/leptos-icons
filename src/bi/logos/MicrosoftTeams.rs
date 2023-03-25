#[cfg(feature = "BiLogosMicrosoftTeams")]
use leptos::*;
#[cfg(feature = "BiLogosMicrosoftTeams")]
///This icon requires the feature `BiLogosMicrosoftTeams` to be enabled.
#[component]
pub fn MicrosoftTeams(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "20.288" cy = "8.344" r =
        "1.707" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.581 11.513h3.413v3.656c0 .942-.765 1.706-1.707 1.706h-1.706v-5.362zM2.006 4.2v15.6l11.213 1.979V2.221L2.006 4.2zm8.288 5.411-1.95.049v5.752H6.881V9.757l-1.949.098V8.539l5.362-.292v1.364zm3.899.439v8.288h1.95c.808 0 1.463-.655 1.463-1.462V10.05h-3.413zm1.463-4.875c-.586 0-1.105.264-1.463.673v2.555c.357.409.877.673 1.463.673a1.95 1.95 0 0 0 0-3.901z"
        /> < title > { title } < / title > < / svg >
    }
}
