// wadhu.com — page DSL. Tokens from docs/DESIGN.md.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let Night = #x0d0b3a
    let Indigo = #x161252
    let Navy = #x0b1f63
    let Cyan = #x00c8fc
    let Lime = #xadfb15
    let Ink = #xfcfbfb
    let Muted = #x9fb4d0

    mod.widgets.Label.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Button.draw_text.sdf_luma_bias = 0.0

    let Eyebrow = Label{
        width: Fill height: Fit
        draw_text +: { color: Lime text_style: theme.font_bold{font_size: 12.} }
    }
    let Headline = Label{
        width: Fill height: Fit
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 36. line_spacing: 1.05} }
    }
    let Lede = Label{
        width: Fill height: Fit
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 17. line_spacing: 1.4} }
    }
    let Body = Label{
        width: Fill height: Fit
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 15. line_spacing: 1.45} }
    }
    let CardTitle = Label{
        width: Fill height: Fit
        draw_text +: { color: Cyan text_style: theme.font_bold{font_size: 18.} }
    }
    let LinkBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 14. right: 14. top: 10. bottom: 10.}
        text: "Open"
        draw_bg +: { color: Cyan }
        draw_text +: { color: Night text_style: theme.font_bold{font_size: 14.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 12. right: 12. top: 8. bottom: 8.}
        draw_bg +: { color: Indigo }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    let Dot = Button{
        width: 14. height: 14.
        text: ""
        draw_bg +: { color: Indigo }
    }
    let Panel = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 18. right: 18. top: 16. bottom: 16.}
        draw_bg +: { color: Indigo }
        flow: Down spacing: 8.
    }
    let Chapter = SolidView{
        width: Fill height: Fill
        padding: Inset{left: 36. right: 36. top: 40. bottom: 36.}
        draw_bg +: { color: Night }
        flow: Down spacing: 14.
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Wadhu — Tools · Apps · More"
                window.inner_size: vec2(1100, 720)
                body +: {
                    page := View{
                        width: Fill height: Fill
                        flow: Down
                        selection_root: true
                        select_from_gaps: true
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 14. bottom: 12.}
                            draw_bg +: { color: Navy }
                            flow: Right spacing: 12.
                            align: Align{y: 0.5}
                            brand := Eyebrow{ text: "WADHU" }
                            <View>{ width: Fill }
                            nav_privacy := GhostBtn{ text: "Privacy" }
                            nav_terms := GhostBtn{ text: "Terms" }
                            nav_connect := LinkBtn{ text: "Connect" }
                        }
                        slides := SlidesView{
                            width: Fill height: Fill
                            axis: Vertical
                            hero := Chapter{
                                <Eyebrow>{ text: "TOOLS · APPS · MORE — BY GOTIGIN" }
                                <Headline>{ text: "Wadhu means more." }
                                <Lede>{ text: "Reusable tools for Gotigin brands and everyone else — SMS on your phone, Cloud messaging onboarding for your customers, and more." }
                                cta_connect := LinkBtn{ text: "Open Wadhu Connect" }
                            }
                            products := Chapter{
                                <Eyebrow>{ text: "PRODUCTS" }
                                <Headline>{ text: "What ships today" }
                                sms := Panel{
                                    <CardTitle>{ text: "SMS Gateway" }
                                    <Body>{ text: "Android-first SIM gateway. LAN and Bluetooth stay free forever." }
                                }
                                conn := Panel{
                                    <CardTitle>{ text: "Wadhu Connect" }
                                    <Body>{ text: "Tech Provider bridge for WhatsApp Cloud API. Shops keep their number; your apps send after Embedded Signup." }
                                }
                            }
                            connect := Chapter{
                                <Eyebrow>{ text: "WADHU CONNECT" }
                                <Headline>{ text: "Their number. Your software." }
                                <Body>{ text: "Brand apps call Connect so a business can grant access to their WhatsApp Business Account. Host: connect.wadhu.com" }
                                <Body>{ text: "We do not take over their chats — we provide the Cloud API pipe." }
                            }
                            about := Chapter{
                                <Eyebrow>{ text: "NAMES" }
                                <Headline>{ text: "More · Find it" }
                                <Body>{ text: "Wadhu means more in Gujarati (and Kutchi). Gotigin means find it in Memon — said go-tee-geen." }
                                <Body>{ text: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED · mearaj@gotigin.com" }
                            }
                            close := Chapter{
                                <Eyebrow>{ text: "NEXT" }
                                <Headline>{ text: "Build with Wadhu." }
                                <Body>{ text: "Cloud accounts on wadhu.com are for hosted features. Local SMS stays free." }
                                <Body>{ text: "Privacy, Terms, and About are real pages on this domain." }
                            }
                        }
                        dots := View{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 8. bottom: 14.}
                            flow: Right spacing: 10.
                            align: Align{x: 0.5, y: 0.5}
                            d0 := Dot{}
                            d1 := Dot{}
                            d2 := Dot{}
                            d3 := Dot{}
                            d4 := Dot{}
                        }
                    }
                }
            }
        }
    }
}
