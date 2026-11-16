const statusEl = document.getElementById('status');
const logEl = document.getElementById('log');
const readyMsg = document.getElementById('ready-msg');
const launchBtn = document.getElementById('launch');
const refreshBtn = document.getElementById('refresh');
const sendForm = document.getElementById('send-form');
const sendResult = document.getElementById('send-result');
const pathLabel = document.getElementById('path-label');

let config = null;
let lastSession = null;

/** Never paint phone numbers or tokens into the on-page dumps. */
function redact(value) {
  const json = JSON.parse(JSON.stringify(value ?? null));
  const phoneKeys = new Set([
    'displayPhoneNumber',
    'phoneE164',
    'phone_number',
    'phoneNumber',
    'to',
    'wa_id',
    'recipient',
  ]);
  const secretKeys = new Set([
    'businessToken',
    'accessToken',
    'token',
    'code',
    'appSecret',
  ]);
  const walk = (node) => {
    if (!node || typeof node !== 'object') return;
    if (Array.isArray(node)) {
      node.forEach(walk);
      return;
    }
    for (const [k, v] of Object.entries(node)) {
      if (phoneKeys.has(k) && v != null && v !== '') {
        node[k] = '[redacted]';
      } else if (secretKeys.has(k) && v != null && v !== '') {
        node[k] = '[redacted]';
      } else if (typeof v === 'string' && /^\+?\d{8,15}$/.test(v.replace(/[\s-]/g, ''))) {
        node[k] = '[redacted]';
      } else {
        walk(v);
      }
    }
  };
  walk(json);
  return json;
}

function showStatus(value) {
  statusEl.textContent = JSON.stringify(redact(value), null, 2);
}

function log(label, value) {
  const line = `[${new Date().toISOString()}] ${label}\n${
    typeof value === 'string' ? value : JSON.stringify(redact(value), null, 2)
  }\n\n`;
  logEl.textContent = line + logEl.textContent;
}

function setPathLabel() {
  if (!pathLabel) return;
  const host = location.host || 'connect.wadhu.com';
  const path = location.pathname === '/' ? '/onboard' : location.pathname;
  pathLabel.textContent = `${host}${path}`;
  // Keep the address bar on /onboard when opened at /
  if (location.pathname === '/' || location.pathname === '') {
    history.replaceState(null, '', '/onboard');
    pathLabel.textContent = `${host}/onboard`;
  }
  const back = document.getElementById('back-wadhu');
  if (back) {
    if (location.hostname === '127.0.0.1' || location.hostname === 'localhost') {
      back.href = `${location.protocol}//${location.hostname}:8020/`;
    } else {
      back.href = 'https://wadhu.com/';
    }
  }
}

async function api(path, options) {
  const res = await fetch(path, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });
  const json = await res.json().catch(() => ({}));
  return { ok: res.ok, status: res.status, json };
}

async function refreshStatus() {
  const { json } = await api('/api/status');
  showStatus(json);
}

async function init() {
  setPathLabel();
  document.title = 'Wadhu Connect · Onboard';

  const { json } = await api('/api/config');
  config = json;
  showStatus(json);

  if (!json.ready) {
    readyMsg.textContent = `Server needs ${(json.missing || []).join(', ')} in .env before launch.`;
    launchBtn.disabled = true;
    return;
  }

  const coexistence =
    new URLSearchParams(location.search).get('coexistence') === '1';
  readyMsg.textContent = coexistence
    ? 'Ready. Launch opens coexistence Embedded Signup (WhatsApp Business app number).'
    : 'Ready. Launch opens Cloud API Embedded Signup. Use business number +91 75740 67702 — not the personal WhatsApp Business app number.';
  config.coexistenceEnabled = coexistence;
  launchBtn.disabled = false;

  window.fbAsyncInit = function fbAsyncInit() {
    window.FB.init({
      appId: json.appId,
      autoLogAppEvents: true,
      xfbml: true,
      version: json.graphVersion,
    });
  };

  if (window.FB) window.fbAsyncInit();
}

window.addEventListener('message', async (event) => {
  if (!String(event.origin || '').endsWith('facebook.com')) return;
  let data = event.data;
  try {
    data = typeof data === 'string' ? JSON.parse(data) : data;
  } catch {
    // keep raw
  }
  if (!data || data.type !== 'WA_EMBEDDED_SIGNUP') return;

  lastSession = data;
  log('WA_EMBEDDED_SIGNUP session', data);
  await api('/api/session', { method: 'POST', body: JSON.stringify(data) });
  await refreshStatus();
});

async function completeOnboarding(code) {
  const payload = {
    code,
    phoneNumberId: lastSession?.data?.phone_number_id,
    wabaId: lastSession?.data?.waba_id,
    businessId: lastSession?.data?.business_id,
    finishEvent: lastSession?.event,
  };
  log('Exchanging code (server-side)', {
    ...payload,
    code: '[redacted]',
    codeLength: code?.length,
  });
  const { ok, json } = await api('/api/complete', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
  log(ok ? 'Onboarding complete' : 'Onboarding failed', json);
  await refreshStatus();
}

function launchWhatsAppSignup() {
  if (!config?.configId) return;
  window.FB.login(
    (response) => {
      if (response?.authResponse?.code) {
        log('FB.login authResponse received', { hasCode: true });
        completeOnboarding(response.authResponse.code);
      } else {
        log('FB.login without code', {
          status: response?.status,
          hasAuthResponse: Boolean(response?.authResponse),
        });
      }
    },
    {
      config_id: config.configId,
      response_type: 'code',
      override_default_response_type: true,
      extras: (() => {
        const extras = {
          setup: {},
          sessionInfoVersion: '3',
        };
        // Only when ?coexistence=1 — Meta #3441107 until Tech Provider clears biz-app numbers.
        if (config?.coexistenceEnabled) {
          extras.featureType = 'whatsapp_business_app_onboarding';
        }
        return extras;
      })(),
    },
  );
}

launchBtn.addEventListener('click', launchWhatsAppSignup);
refreshBtn.addEventListener('click', refreshStatus);

sendForm.addEventListener('submit', async (e) => {
  e.preventDefault();
  const fd = new FormData(sendForm);
  const body = {
    to: String(fd.get('to') || '').replace(/\D/g, ''),
    text: String(fd.get('text') || ''),
  };
  const { json } = await api('/api/send-test', {
    method: 'POST',
    body: JSON.stringify(body),
  });
  sendResult.textContent = JSON.stringify(redact(json), null, 2);
});

init().catch((err) => {
  statusEl.textContent = String(err);
});
