#[cfg(feature = "SiApachenetbeanside")]
use leptos::*;
#[cfg(feature = "SiApachenetbeanside")]
///This icon requires the feature `SiApachenetbeanside` to be enabled.
#[component]
pub fn Apachenetbeanside(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M22.8575 6.1211c-.0092-.0866-.0486-.1598-.121-.2104C22.7381 5.9098 12.1194.032 12.1185.03a.2487.2487 0 00-.2372 0c-.003 0-10.6129 5.8784-10.6145 5.8797-.0744.0497-.1156.1241-.1243.2124v11.8365c.0149.0565.0178.1253.071.161.0183.0788 10.6175 5.7985 10.6694 5.8507a.2456.2456 0 00.2343 0c.0025-.0017 10.6135-5.7957 10.6147-5.7984.0748-.0489.115-.1262.1256-.2133V6.1211zm-17.633 9.3292L1.6323 17.534V6.5401l3.5922 2.011zm13.551-6.8993l3.5922-2.0109v10.9938l-3.5922-2.0836zm-.7407-.1455c-2.0115 1.1316-4.0232 2.263-6.0348 3.3943a125706.698 125706.698 0 01-6.0349-3.3943L12 4.9349zM5.7143 15.4473V8.8265l6.0408 3.398v6.5444l-6.0408-3.3216zm6.5305 3.3216v-6.5444l6.0409-3.398v6.6208c-2.0135 1.1074-4.0272 2.2145-6.0408 3.3216zM12 .5247l10.1094 5.5984-3.5771 2.003-6.4104-3.6858a.2418.2418 0 00-.2439 0L5.4676 8.126l-3.577-2.003L12 .5248zM5.4728 15.8731l6.2823 3.4547v4.014L1.8868 17.952zm6.772 7.4686v-4.014l6.2824-3.4546 3.586 2.0789z"
        /> < title > { title } < / title > < / svg >
    }
}
