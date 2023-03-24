#[cfg(feature = "TiPipette")]
use leptos::*;
#[cfg(feature = "TiPipette")]
///This icon requires the feature `TiPipette` to be enabled.
#[component]
pub fn Pipette(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.384 7.331c.073-1.199-.354-2.388-1.146-3.179-.732-.731-1.793-1.152-2.912-1.152-1.176 0-2.206.453-2.825 1.243-.692.883-1.392 2.625-1.769 3.647l-1.616-1.617c-.392-.391-1.023-.391-1.414 0-.392.392-.392 1.023 0 1.414l.293.293-5.231 5.232c-.375.375-.719.912-.968 1.516-.019.043-1.726 4.328-.093 5.959.527.526 1.33.707 2.178.707 1.778-.002 3.753-.787 3.783-.801.602-.248 1.141-.592 1.514-.967l5.232-5.232.293.293c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.022 0-1.414l-1.617-1.616c1.023-.376 2.766-1.075 3.648-1.769.721-.562 1.17-1.493 1.236-2.557zm-16.265 11.944c-.247-.295-.105-1.508.154-2.58l2.422 2.423c-1.071.261-2.283.403-2.576.157zm4.645-1.061c-.188.188-.511.388-.865.533l-.116.042-3.181-3.18.043-.117c.146-.354.346-.678.533-.864l5.232-5.231 3.586 3.586-5.232 5.231z"
        /> < title > { title } < / title > < / svg >
    }
}
