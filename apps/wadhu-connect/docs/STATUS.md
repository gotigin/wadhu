# Wadhu Connect — status

Updated: 2026-11-15  
Home: `apps/wadhu-connect` (Rust). Legacy Node: `apps/gotigin-connect`.

## Hosts

| | |
| --- | --- |
| Brand / privacy | https://wadhu.com · https://wadhu.com/privacy |
| Connect | https://connect.wadhu.com |
| Local | `cargo wadhu-connect-dev` → http://127.0.0.1:3847 |

## Meta App Review

Paste answers: [`DATA_HANDLING_AND_REVIEWER.md`](DATA_HANDLING_AND_REVIEWER.md)

**Do in Meta dashboard (manual):**

1. App settings → Basic → rename display name to **Wadhu Connect**
2. Privacy + data deletion → `https://wadhu.com/privacy` (after Firebase deploy)
3. Website Site URL → `https://wadhu.com`
4. WhatsApp webhook → `https://connect.wadhu.com/webhook` (when TLS host is up)
5. Complete Allowed usage + Data handling + Reviewer instructions from the paste doc
6. Submit submission `1606022664070459`

Until DNS is live, temporary tunnels may point at local `:3847` for webhook/ES only — never use khaynjo privacy as the Wadhu product URL.

## IDs (unchanged)

- App `1606021637403895`
- Config `1895670411865695`
- Test phone `+1 555-156-0957` / `1408752645660951`
- WABA `1324845626382929`
