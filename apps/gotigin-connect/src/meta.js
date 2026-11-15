/**
 * Minimal Meta Graph helpers. Secrets stay server-side.
 */

export function createMetaClient({ appId, appSecret, graphVersion }) {
  const base = `https://graph.facebook.com/${graphVersion}`;

  async function graph(pathname, { method = 'GET', token, body, form } = {}) {
    const url = new URL(`${base}${pathname}`);
    const headers = {};
    let payload;

    if (token) headers.Authorization = `Bearer ${token}`;

    if (form) {
      payload = new URLSearchParams(form);
      headers['Content-Type'] = 'application/x-www-form-urlencoded';
    } else if (body !== undefined) {
      payload = JSON.stringify(body);
      headers['Content-Type'] = 'application/json';
    }

    const res = await fetch(url, { method, headers, body: payload });
    const text = await res.text();
    let json;
    try {
      json = text ? JSON.parse(text) : {};
    } catch {
      json = { raw: text };
    }
    return { ok: res.ok, status: res.status, json };
  }

  async function exchangeCode(code, redirectUri) {
    return graph('/oauth/access_token', {
      method: 'GET',
      form: undefined,
      // GET with query params
    }).then(async () => {
      const url = new URL(`${base}/oauth/access_token`);
      url.searchParams.set('client_id', appId);
      url.searchParams.set('client_secret', appSecret);
      url.searchParams.set('code', code);
      if (redirectUri) url.searchParams.set('redirect_uri', redirectUri);
      const res = await fetch(url);
      const json = await res.json();
      return { ok: res.ok, status: res.status, json };
    });
  }

  async function subscribeApp(wabaId, token) {
    return graph(`/${wabaId}/subscribed_apps`, { method: 'POST', token });
  }

  async function registerPhone(phoneNumberId, token, pin) {
    return graph(`/${phoneNumberId}/register`, {
      method: 'POST',
      token,
      body: { messaging_product: 'whatsapp', pin },
    });
  }

  async function phoneStatus(phoneNumberId, token) {
    return graph(`/${phoneNumberId}`, {
      method: 'GET',
      token,
      // fields via query
    }).then(async () => {
      const url = new URL(`${base}/${phoneNumberId}`);
      url.searchParams.set(
        'fields',
        'display_phone_number,verified_name,quality_rating,platform_type,is_on_biz_app,code_verification_status,status',
      );
      const res = await fetch(url, {
        headers: { Authorization: `Bearer ${token}` },
      });
      const json = await res.json();
      return { ok: res.ok, status: res.status, json };
    });
  }

  async function sendText(phoneNumberId, token, to, body) {
    return graph(`/${phoneNumberId}/messages`, {
      method: 'POST',
      token,
      body: {
        messaging_product: 'whatsapp',
        recipient_type: 'individual',
        to,
        type: 'text',
        text: { preview_url: false, body },
      },
    });
  }

  async function requestHistorySync(phoneNumberId, token) {
    return graph(`/${phoneNumberId}/smb_app_data`, {
      method: 'POST',
      token,
      body: {
        messaging_product: 'whatsapp',
        sync_type: 'smb_app_state_sync',
      },
    });
  }

  return {
    exchangeCode,
    subscribeApp,
    registerPhone,
    phoneStatus,
    sendText,
    requestHistorySync,
  };
}
