#[cfg(feature = "SiOpsgenie")]
use leptos::*;
#[cfg(feature = "SiOpsgenie")]
///This icon requires the feature `SiOpsgenie` to be enabled.
#[component]
pub fn Opsgenie(
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
        "M12.002 0a5.988 5.988 0 1 1 0 11.975 5.988 5.988 0 0 1 0-11.975zm9.723 13.026h-.03l-4.527-2.242a.671.671 0 0 0-.876.268 22.408 22.408 0 0 1-4.306 5.217 22.407 22.407 0 0 1-4.286-5.2.671.671 0 0 0-.876-.269l-4.535 2.226h-.03a.671.671 0 0 0-.248.902 28.85 28.85 0 0 0 4.55 5.933l-.002.001c.024.025.05.048.075.072.335.335.676.664 1.027.981.081.074.165.144.247.217.315.278.632.555.96.82.144.117.295.227.441.341.277.216.552.434.837.639.44.318.888.625 1.346.917a.963.963 0 0 0 1.007.017c.487-.312.962-.64 1.428-.98.068-.05.132-.103.2-.153.358-.266.713-.537 1.06-.82.234-.19.46-.39.688-.588.17-.147.34-.291.506-.442.295-.268.58-.545.864-.825.061-.06.127-.118.188-.179l-.004-.002a28.852 28.852 0 0 0 4.565-5.949.671.671 0 0 0-.269-.902z"
        /> < title > { title } < / title > < / svg >
    }
}
