const statusEl = document.getElementById('status');
const logEl = document.getElementById('log');
const readyMsg = document.getElementById('ready-msg');
const launchBtn = document.getElementById('launch');
const refreshBtn = document.getElementById('refresh');
const sendForm = document.getElementById('send-form');
const sendResult = document.getElementById('send-result');

let config = null;
let lastSession = null;

function log(label, value) {
  const line = `[${new Date().toISOString()}] ${label}\n${typeof value === 'string' ? value : JSON.stringify(value, null, 2)}\n\n`;
  logEl.textContent = line + logEl.textContent;
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
  statusEl.textContent = JSON.stringify(json, null, 2);
}

async function init() {
  const { json } = await api('/api/config');
  config = json;
  statusEl.textContent = JSON.stringify(json, null, 2);

  if (!json.ready) {
    readyMsg.textContent = `Server needs ${json.missing.join(', ')} in .env before launch.`;
    launchBtn.disabled = true;
    return;
  }

  readyMsg.textContent =
    'SDK will launch coexistence Embedded Signup (featureType=whatsapp_business_app_onboarding).';
  launchBtn.disabled = false;

  window.fbAsyncInit = function fbAsyncInit() {
    window.FB.init({
      appId: json.appId,
      autoLogAppEvents: true,
      xfbml: true,
      version: json.graphVersion,
    });
  };

  // If SDK already loaded before this assignment, init now.
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
    code: '[redacted in UI log length]',
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
      // Never print full auth payloads with tokens/codes to the visible page long-term;
      // show only success/failure shape here.
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
      extras: {
        setup: {},
        featureType: 'whatsapp_business_app_onboarding',
        sessionInfoVersion: '3',
      },
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
  sendResult.textContent = JSON.stringify(json, null, 2);
});

init().catch((err) => {
  statusEl.textContent = String(err);
});
