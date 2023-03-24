#[cfg(feature = "SiTubi")]
use leptos::*;
#[cfg(feature = "SiTubi")]
///This icon requires the feature `SiTubi` to be enabled.
#[component]
pub fn Tubi(
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
        "M23.7692 9.6906h-1.6154v7.159c0 .1276.1034.231.2308.231h1.3846a.2309.2309 0 0 0 .2308-.231v-6.928a.2309.2309 0 0 0-.2308-.231m-6.2307 5.5425c-1.0196 0-1.8462-.8271-1.8462-1.8475 0-1.0203.8266-1.8475 1.8462-1.8475s1.8461.8272 1.8461 1.8475c0 1.0204-.8265 1.8475-1.8461 1.8475m0-5.5425c-.542 0-1.0566.117-1.5203.327v-.0001a.2307.2307 0 0 1-.3259-.2104V7.1503a.2309.2309 0 0 0-.2308-.2309h-1.6153v6.4662c0 2.0407 1.653 3.695 3.6923 3.695 2.0392 0 3.6923-1.6543 3.6923-3.695 0-2.0406-1.653-3.695-3.6923-3.695m-12.1189 6.689.0002.0002-.6948-1.2052-.0004.0002a.2307.2307 0 0 0-.2925-.096 1.8389 1.8389 0 0 1-.7398.1543c-1.0196 0-1.8462-.8272-1.8462-1.8475v-1.6165c0-.1276.1034-.231.2308-.231h2.3077a.2309.2309 0 0 0 .2308-.231V9.9217a.2309.2309 0 0 0-.2308-.231H2.077a.2309.2309 0 0 1-.2307-.2309V7.1503a.2309.2309 0 0 0-.2308-.2309H0v6.4662c0 2.0407 1.6531 3.695 3.6923 3.695.5849 0 1.138-.136 1.6295-.3783a.231.231 0 0 0 .0978-.3227m7.2727-6.689h-1.6154v3.695c0 1.0203-.8266 1.8475-1.8462 1.8475s-1.8461-.8272-1.8461-1.8475v-3.464a.2309.2309 0 0 0-.2308-.231H5.5384v3.695c0 2.0407 1.6531 3.695 3.6923 3.695s3.6923-1.6543 3.6923-3.695v-3.464a.2309.2309 0 0 0-.2307-.231m10.3846-2.7712c-.5099 0-.9231.4135-.9231.9237s.4132.9238.923.9238c.5099 0 .9232-.4136.9232-.9238s-.4133-.9237-.9231-.9237"
        /> < title > { title } < / title > < / svg >
    }
}
