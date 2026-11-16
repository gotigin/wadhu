// wadhu.com — one scrolling brand page (ScrollYView).
// Naming: Wadhu=brand, Gotigin=trademark, company=GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED.
// Products: SMS Gateway, Wadhu Connect.
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
    let Select = #x24407aff

    mod.widgets.Label.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Button.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.selectable = true
    mod.widgets.Markdown.draw_selection.color = Select
    mod.widgets.TextFlow.draw_selection.color = Select

    // Markdown paints from font_color / text_style_* (Khaynjo); draw_text alone is not enough.
    let Eyebrow = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Lime
        font_size: 13.
        text_style_normal: theme.font_bold{font_size: 13. line_spacing: 1.3}
        text_style_bold: theme.font_bold{font_size: 13. line_spacing: 1.3}
        draw_text +: { color: Lime text_style: theme.font_bold{font_size: 13. line_spacing: 1.3} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Headline = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Ink
        font_size: 48.
        text_style_normal: theme.font_bold{font_size: 48. line_spacing: 1.05}
        text_style_bold: theme.font_bold{font_size: 48. line_spacing: 1.05}
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 48. line_spacing: 1.05} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Lede = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 19.
        text_style_normal: theme.font_regular{font_size: 19. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 19. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 19. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Body = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 16.
        text_style_normal: theme.font_regular{font_size: 16. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 16. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 16. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitle = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Cyan
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.25}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.25}
        draw_text +: { color: Cyan text_style: theme.font_bold{font_size: 22. line_spacing: 1.25} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let LinkBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 18. right: 18. top: 13. bottom: 13.}
        text: "Open"
        draw_bg +: { color: Cyan }
        draw_text +: { color: Night text_style: theme.font_bold{font_size: 14.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 14. right: 14. top: 11. bottom: 11.}
        draw_bg +: { color: Indigo }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    // Full-bleed band; children use the whole window width.
    let Band = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Night }
        flow: Down
        spacing: 18.
    }
    let BandAlt = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Navy }
        flow: Down
        spacing: 18.
    }
    let Panel = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 24. right: 24. top: 22. bottom: 22.}
        draw_bg +: { color: Indigo }
        flow: Down
        spacing: 12.
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Wadhu — Tools · Apps · More"
                window.inner_size: vec2(1280, 800)
                pass.clear_color: Night
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    shell := SolidView{
                        width: Fill height: Fill
                        draw_bg +: { color: Night }
                        flow: Down
                        // One header on every route (home + legal). Logo always here.
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 12. bottom: 12.}
                            draw_bg +: { color: Navy }
                            flow: Right
                            spacing: 10.
                            align: Align{y: 0.5}
                            brand_icon := Image{
                                width: 40 height: 40
                                fit: ImageFit.Stretch
                                src: crate_resource("self:resources/media/wadhu-brand-logo.webp")
                            }
                            // Shown beside the logo when the window is wide enough (apply_responsive).
                            brand_tag := Label{
                                width: Fit
                                text: "WADHU · TOOLS · APPS · MORE"
                                draw_text +: {
                                    color: Lime
                                    text_style: theme.font_bold{font_size: 12.}
                                }
                            }
                            <View>{ width: Fill height: 1 }
                            // Visibility / active pair toggled in Rust. Never set
                            // visible:false in the template — that drops GhostBtn colors.
                            nav_back := GhostBtn{ text: "← Back" }
                            nav_privacy := GhostBtn{ text: "Privacy" }
                            nav_privacy_on := LinkBtn{ text: "Privacy" }
                            nav_terms := GhostBtn{ text: "Terms" }
                            nav_terms_on := LinkBtn{ text: "Terms" }
                            nav_about := GhostBtn{ text: "About" }
                            nav_about_on := LinkBtn{ text: "About" }
                            nav_connect := GhostBtn{ text: "Wadhu Connect" }
                        }
                        // Home + in-app legal share the wasm shell so Privacy/Terms/About
                        // do not unload the canvas (back = instant, no wasm reload).
                        // Static /privacy HTML still exists for Meta / direct links.
                        pages := View{
                            width: Fill
                            height: Fill
                            flow: Overlay

                        page := ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            selection_root: true
                            select_from_gaps: true

                            hero := Band{
                                // Tagline lives in the shared top bar when width allows;
                                // this wrap is the phone/narrow fallback (toggled in Rust).
                                hero_tag := View{
                                    width: Fill height: Fit
                                    Eyebrow{ body: "WADHU · TOOLS · APPS · MORE" }
                                }
                                Headline{ body: "Wadhu means more." }
                                Lede{ body: "Reusable tools under the Wadhu brand — from GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED. Trademark: Gotigin. Products: SMS Gateway and Wadhu Connect." }
                                cta_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    cta_connect := LinkBtn{ text: "Open Wadhu Connect" }
                                    cta_privacy := GhostBtn{ text: "Privacy policy" }
                                }
                            }

                            products := BandAlt{
                                Eyebrow{ body: "PRODUCTS" }
                                Headline{ body: "Under the Wadhu brand" }
                                Lede{ body: "Two products you can run now." }
                                cards := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 18.
                                    sms := Panel{
                                        CardTitle{ body: "SMS Gateway" }
                                        Body{ body: "Android-first SIM gateway. LAN and Bluetooth stay free forever. Optional hosted relay later — not required for local send." }
                                    }
                                    conn := Panel{
                                        conn_mark := Image{
                                            width: 72 height: 72
                                            fit: ImageFit.Stretch
                                            src: crate_resource("self:resources/media/wadhu-connect-logo.webp")
                                        }
                                        CardTitle{ body: "Wadhu Connect" }
                                        Body{ body: "WhatsApp Cloud API Tech Provider bridge. Businesses keep their number; apps send and receive after Embedded Signup." }
                                        open_connect := LinkBtn{ text: "Open Wadhu Connect" }
                                    }
                                }
                            }

                            connect := Band{
                                Eyebrow{ body: "WADHU CONNECT" }
                                Headline{ body: "Their number. Your software." }
                                Body{ body: "Other Gotigin brands (Khaynjo, Moklanjo, and others) call Wadhu Connect so a business can grant access to their WhatsApp Business Account. We provide the Cloud API pipe — we do not take over their chats." }
                                Body{ body: "Host: connect.wadhu.com · Privacy: wadhu.com/privacy" }
                            }

                            about := BandAlt{
                                Eyebrow{ body: "NAMES" }
                                Headline{ body: "Brand · Trademark · Company" }
                                Body{ body: "Wadhu is the brand (more in Gujarati / Kutchi) — Tools · Apps · More." }
                                Body{ body: "Gotigin is the trademark (find it in Memon — go-tee-geen). Slogan: Where Technology Meets Discovery." }
                                Body{ body: "Company: GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED · mearaj@gotigin.com" }
                            }

                            close := Band{
                                Eyebrow{ body: "NEXT" }
                                Headline{ body: "Build with Wadhu." }
                                Body{ body: "Cloud accounts on wadhu.com are for hosted Wadhu features. Local SMS over LAN and Bluetooth stays free." }
                                Body{ body: "Privacy, Terms, and About are in the top bar." }
                            }
                        }

                        legal := ScrollYView{
                            visible: false
                            width: Fill
                            height: Fill
                            flow: Down
                            selection_root: true
                            select_from_gaps: true
                            // Same shared `top` header as home (logo + tag + nav).
                            legal_pad := SolidView{
                                width: Fill height: Fit
                                padding: Inset{left: 64. right: 64. top: 24. bottom: 80.}
                                draw_bg +: { color: Night }
                                flow: Down
                                spacing: 16.
                                legal_md := Markdown{
                                    selectable: true
                                    width: Fill
                                    height: Fit
                                    padding: 0.
                                    paragraph_spacing: 10.
                                    pre_code_spacing: 8.
                                    body: ""
                                    font_color: Muted
                                    font_size: 16.
                                    text_style_normal: theme.font_regular{font_size: 16. line_spacing: 1.45}
                                    text_style_bold: theme.font_bold{font_size: 16. line_spacing: 1.45}
                                    draw_text +: { color: Muted text_style: theme.font_regular{font_size: 16. line_spacing: 1.45} }
                                    draw_selection +: { color: Select }
                                    draw_block +: { selection_color: Select }
                                }
                            }
                        }
                        } // pages
                    }
                }
            }
        }
    }
}
