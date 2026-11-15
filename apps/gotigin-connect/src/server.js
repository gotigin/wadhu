import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { createStore } from './store.js';
import { createMetaClient } from './meta.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, '..');

function loadEnv() {
  const envPath = path.join(root, '.env');
  if (!fs.existsSync(envPath)) return;
  for (const line of fs.readFileSync(envPath, 'utf8').split('\n')) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const eq = trimmed.indexOf('=');
    if (eq < 0) continue;
    const key = trimmed.slice(0, eq).trim();
    let val = trimmed.slice(eq + 1).trim();
    if (
      (val.startsWith('"') && val.endsWith('"')) ||
      (val.startsWith("'") && val.endsWith("'"))
    ) {
      val = val.slice(1, -1);
    }
    if (!(key in process.env)) process.env[key] = val;
  }
}

loadEnv();

const PORT = Number(process.env.PORT || 3847);
const HOST = process.env.HOST || '127.0.0.1';
const DATA_DIR = path.resolve(root, process.env.DATA_DIR || './data');
const PUBLIC_DIR = path.join(root, 'public');

const cfg = {
  appId: process.env.META_APP_ID || '',
  appSecret: process.env.META_APP_SECRET || '',
  configId: process.env.META_CONFIG_ID || '',
  graphVersion: process.env.META_GRAPH_VERSION || 'v22.0',
  verifyToken: process.env.WEBHOOK_VERIFY_TOKEN || 'gotigin_wa_verify_change_me',
  publicOrigin: process.env.PUBLIC_ORIGIN || `http://${HOST}:${PORT}`,
};

const store = createStore(DATA_DIR);
const meta = createMetaClient(cfg);

function json(res, status, body) {
  const payload = JSON.stringify(body, null, 2);
  res.writeHead(status, {
    'Content-Type': 'application/json; charset=utf-8',
    'Cache-Control': 'no-store',
  });
  res.end(payload);
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on('data', (c) => chunks.push(c));
    req.on('end', () => resolve(Buffer.concat(chunks)));
    req.on('error', reject);
  });
}

function verifyWebhookSignature(rawBody, signatureHeader) {
  if (!cfg.appSecret) return { ok: false, reason: 'META_APP_SECRET not set' };
  if (!signatureHeader?.startsWith('sha256=')) {
    return { ok: false, reason: 'missing X-Hub-Signature-256' };
  }
  const expected = crypto
    .createHmac('sha256', cfg.appSecret)
    .update(rawBody)
    .digest('hex');
  const provided = signatureHeader.slice('sha256='.length);
  const a = Buffer.from(expected, 'hex');
  const b = Buffer.from(provided, 'hex');
  if (a.length !== b.length || !crypto.timingSafeEqual(a, b)) {
    return { ok: false, reason: 'signature mismatch' };
  }
  return { ok: true };
}

function publicConfig() {
  const connection = store.readConnection();
  const ready = Boolean(cfg.appId && cfg.configId);
  return {
    ready,
    missing: [
      !cfg.appId && 'META_APP_ID',
      !cfg.appSecret && 'META_APP_SECRET',
      !cfg.configId && 'META_CONFIG_ID',
    ].filter(Boolean),
    appId: cfg.appId || null,
    configId: cfg.configId || null,
    graphVersion: cfg.graphVersion,
    publicOrigin: cfg.publicOrigin,
    webhookPath: '/webhook',
    coexistence: {
      featureType: 'whatsapp_business_app_onboarding',
      sessionInfoVersion: '3',
    },
    knownAssets: {
      businessPortfolioId: '1076913081550690',
      facebookPageId: '1233972929807232',
      messagingAccountMearajId: '147378475125876',
      businessAppPhoneNumberId: '152212324639075',
      incompleteWhatsAppAccountAssetId: '1008869008221270',
      phoneE164: '+919967717702',
    },
    connection: connection
      ? {
          status: connection.status || null,
          wabaId: connection.wabaId || null,
          phoneNumberId: connection.phoneNumberId || null,
          businessId: connection.businessId || null,
          platformType: connection.platformType || null,
          isOnBizApp: connection.isOnBizApp ?? null,
          finishEvent: connection.finishEvent || null,
          updatedAt: connection.updatedAt || null,
          hasBusinessToken: Boolean(connection.businessToken),
        }
      : null,
  };
}

async function handleApi(req, res, url) {
  if (req.method === 'GET' && url.pathname === '/api/config') {
    return json(res, 200, publicConfig());
  }

  if (req.method === 'GET' && url.pathname === '/api/status') {
    const connection = store.readConnection();
    if (!connection?.businessToken || !connection?.phoneNumberId) {
      return json(res, 200, {
        connected: false,
        reason: 'No Cloud API connection stored yet',
        connection: publicConfig().connection,
        recentWebhooks: store.recentWebhooks(10),
      });
    }
    const status = await meta.phoneStatus(
      connection.phoneNumberId,
      connection.businessToken,
    );
    if (status.ok) {
      store.writeConnection({
        platformType: status.json.platform_type,
        isOnBizApp: status.json.is_on_biz_app,
        displayPhoneNumber: status.json.display_phone_number,
        verifiedName: status.json.verified_name,
        qualityRating: status.json.quality_rating,
        phoneStatus: status.json.status,
      });
    }
    return json(res, 200, {
      connected: status.ok,
      graph: status,
      connection: publicConfig().connection,
      recentWebhooks: store.recentWebhooks(10),
    });
  }

  if (req.method === 'POST' && url.pathname === '/api/session') {
    const raw = await readBody(req);
    const body = raw.length ? JSON.parse(raw.toString('utf8')) : {};
    store.logSession(body);
    if (body?.data?.phone_number_id || body?.data?.waba_id) {
      store.writeConnection({
        status: body.event || 'session',
        phoneNumberId: body.data.phone_number_id || undefined,
        wabaId: body.data.waba_id || undefined,
        businessId: body.data.business_id || undefined,
        finishEvent: body.event || undefined,
      });
    }
    return json(res, 200, { ok: true });
  }

  if (req.method === 'POST' && url.pathname === '/api/complete') {
    if (!cfg.appId || !cfg.appSecret) {
      return json(res, 400, {
        ok: false,
        error: 'META_APP_ID / META_APP_SECRET not configured on server',
      });
    }
    const raw = await readBody(req);
    const body = raw.length ? JSON.parse(raw.toString('utf8')) : {};
    const { code, phoneNumberId, wabaId, businessId, finishEvent } = body;
    if (!code) {
      return json(res, 400, { ok: false, error: 'Missing authorization code' });
    }

    const exchanged = await meta.exchangeCode(code);
    if (!exchanged.ok || !exchanged.json.access_token) {
      store.writeConnection({
        status: 'token_exchange_failed',
        lastError: exchanged.json,
      });
      return json(res, 502, {
        ok: false,
        step: 'token_exchange',
        graph: exchanged,
      });
    }

    const businessToken = exchanged.json.access_token;
    const conn = store.writeConnection({
      status: 'token_exchanged',
      businessToken,
      phoneNumberId: phoneNumberId || undefined,
      wabaId: wabaId || undefined,
      businessId: businessId || undefined,
      finishEvent: finishEvent || undefined,
      tokenExpiresIn: exchanged.json.expires_in,
    });

    const results = { tokenExchange: { ok: true, expiresIn: exchanged.json.expires_in } };

    if (conn.wabaId) {
      results.subscribe = await meta.subscribeApp(conn.wabaId, businessToken);
    }

    // Coexistence numbers often still need /register after Embedded Signup.
    if (conn.phoneNumberId) {
      const pin = String(Math.floor(100000 + Math.random() * 900000));
      results.register = await meta.registerPhone(
        conn.phoneNumberId,
        businessToken,
        pin,
      );
      // Keep PIN only in memory response for operator; do not persist.
      results.registerPinUsedOnce = pin;
      results.phoneStatus = await meta.phoneStatus(
        conn.phoneNumberId,
        businessToken,
      );
      if (results.phoneStatus.ok) {
        store.writeConnection({
          status: 'cloud_api_connected',
          platformType: results.phoneStatus.json.platform_type,
          isOnBizApp: results.phoneStatus.json.is_on_biz_app,
          displayPhoneNumber: results.phoneStatus.json.display_phone_number,
          verifiedName: results.phoneStatus.json.verified_name,
        });
      }
      results.historySync = await meta.requestHistorySync(
        conn.phoneNumberId,
        businessToken,
      );
    }

    return json(res, 200, {
      ok: true,
      connection: publicConfig().connection,
      results,
    });
  }

  if (req.method === 'POST' && url.pathname === '/api/send-test') {
    const connection = store.readConnection();
    if (!connection?.businessToken || !connection?.phoneNumberId) {
      return json(res, 400, { ok: false, error: 'Not connected' });
    }
    const raw = await readBody(req);
    const body = raw.length ? JSON.parse(raw.toString('utf8')) : {};
    const to = body.to;
    const text =
      body.text ||
      `Gotigin Cloud API test ${new Date().toISOString()}`;
    if (!to) {
      return json(res, 400, {
        ok: false,
        error: 'Provide { "to": "<E.164 digits without +>" }',
      });
    }
    const sent = await meta.sendText(
      connection.phoneNumberId,
      connection.businessToken,
      to,
      text,
    );
    return json(res, sent.ok ? 200 : 502, { ok: sent.ok, graph: sent });
  }

  return json(res, 404, { error: 'Not found' });
}

async function handleWebhook(req, res, url) {
  if (req.method === 'GET') {
    const mode = url.searchParams.get('hub.mode');
    const token = url.searchParams.get('hub.verify_token');
    const challenge = url.searchParams.get('hub.challenge');
    if (mode === 'subscribe' && token === cfg.verifyToken) {
      res.writeHead(200, { 'Content-Type': 'text/plain' });
      res.end(challenge || '');
      return;
    }
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }

  if (req.method === 'POST') {
    const raw = await readBody(req);
    const sig = req.headers['x-hub-signature-256'];
    const verified = verifyWebhookSignature(raw, sig);
    let payload;
    try {
      payload = JSON.parse(raw.toString('utf8') || '{}');
    } catch {
      payload = { parseError: true, raw: raw.toString('utf8').slice(0, 500) };
    }
    store.logWebhook({
      verified: verified.ok,
      verifyReason: verified.reason,
      payload,
    });
    // Always 200 quickly so Meta does not retry aggressively during setup.
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ success: true }));
    return;
  }

  res.writeHead(405);
  res.end();
}

function contentType(filePath) {
  if (filePath.endsWith('.html')) return 'text/html; charset=utf-8';
  if (filePath.endsWith('.js')) return 'text/javascript; charset=utf-8';
  if (filePath.endsWith('.css')) return 'text/css; charset=utf-8';
  if (filePath.endsWith('.json')) return 'application/json; charset=utf-8';
  return 'application/octet-stream';
}

function serveStatic(res, filePath) {
  if (!fs.existsSync(filePath) || !fs.statSync(filePath).isFile()) {
    res.writeHead(404);
    res.end('Not found');
    return;
  }
  res.writeHead(200, { 'Content-Type': contentType(filePath) });
  fs.createReadStream(filePath).pipe(res);
}

const server = http.createServer(async (req, res) => {
  try {
    const url = new URL(req.url || '/', `http://${req.headers.host}`);
    if (url.pathname.startsWith('/api/')) {
      await handleApi(req, res, url);
      return;
    }
    if (url.pathname === '/webhook') {
      await handleWebhook(req, res, url);
      return;
    }
    const rel =
      url.pathname === '/' ? '/index.html' : path.normalize(url.pathname);
    if (rel.includes('..')) {
      res.writeHead(400);
      res.end('Bad path');
      return;
    }
    serveStatic(res, path.join(PUBLIC_DIR, rel));
  } catch (err) {
    console.error(err);
    json(res, 500, { error: String(err?.message || err) });
  }
});

server.listen(PORT, HOST, () => {
  console.log(`Gotigin WhatsApp coexistence server on http://${HOST}:${PORT}`);
  console.log(`Public origin: ${cfg.publicOrigin}`);
  console.log(
    `Config ready: ${Boolean(cfg.appId && cfg.configId && cfg.appSecret)} (missing secrets stay empty until .env is filled)`,
  );
});
