// Unified register / sign-in for wadhu.com (Firebase Auth).
import { auth } from "./firebase-init.js";
import {
  GoogleAuthProvider,
  GithubAuthProvider,
  OAuthProvider,
  RecaptchaVerifier,
  createUserWithEmailAndPassword,
  signInWithEmailAndPassword,
  sendEmailVerification,
  signInWithPopup,
  signInWithPhoneNumber,
  signOut,
  onAuthStateChanged,
  updateProfile,
} from "firebase/auth";

const $ = (id) => document.getElementById(id);
const T = (key, english) => (window.siteT ? window.siteT(key, english) : english);

const els = {
  gate: $("auth-gate"),
  session: $("auth-session"),
  status: $("auth-status"),
  error: $("auth-error"),
  email: $("auth-email"),
  password: $("auth-password"),
  displayName: $("auth-display-name"),
  phone: $("auth-phone"),
  smsCode: $("auth-sms-code"),
  phoneStep: $("auth-phone-step"),
  smsStep: $("auth-sms-step"),
  userEmail: $("session-email"),
  userVerified: $("session-verified"),
};

let confirmationResult = null;
let recaptcha = null;
let lastErr = null;

function showError(msg) {
  els.error.textContent = msg || "";
  els.error.hidden = !msg;
}

function reportError(err) {
  lastErr = err || null;
  showError(err ? friendlyError(err) : "");
}

function setBusy(busy) {
  els.gate?.querySelectorAll("button, input").forEach((el) => {
    el.disabled = !!busy;
  });
}

function friendlyError(err) {
  const code = err?.code || "";
  const map = {
    "auth/invalid-email": T("err_invalid_email", "Enter a valid email address."),
    "auth/weak-password": T("err_weak_password", "Use a password with at least 6 characters."),
    "auth/wrong-password": T("err_wrong_password", "Wrong password."),
    "auth/invalid-credential": T("err_invalid_credential", "Email or password is wrong."),
    "auth/popup-closed-by-user": T("err_popup_closed", "Sign-in window was closed."),
    "auth/cancelled-popup-request": T("err_cancelled", "Sign-in was cancelled."),
    "auth/unauthorized-domain": T("err_unauthorized_domain", "This domain is not authorized for Firebase Auth."),
    "auth/operation-not-allowed": T("err_not_allowed", "That sign-in method is not enabled yet."),
    "auth/too-many-requests": T("err_too_many", "Too many attempts. Try again later."),
    "auth/invalid-phone-number": T("err_invalid_phone", "Use a full phone number with country code (e.g. +91…)."),
    "auth/invalid-verification-code": T("err_bad_code", "That SMS code is wrong or expired."),
  };
  return map[code] || err?.message || T("err_generic", "Something went wrong.");
}

function renderSession(user) {
  if (user) {
    els.gate.hidden = true;
    els.session.hidden = false;
    els.userEmail.textContent = user.email || user.phoneNumber || T("acct_no_email", "(no email)");
    els.userVerified.textContent = user.email
      ? user.emailVerified
        ? T("acct_email_verified", "Email verified")
        : T("acct_email_unverified", "Email not verified yet — check your inbox")
      : user.phoneNumber
        ? T("acct_phone_signed_in", "Signed in with phone")
        : T("acct_signed_in", "Signed in");
    els.status.textContent = T("acct_you_are_in", "You’re signed in.");
  } else {
    els.gate.hidden = false;
    els.session.hidden = true;
    els.status.textContent = T("acct_continue_with", "Continue with email, a provider, or phone.");
  }
}

async function afterAuth(user, { sendVerify } = {}) {
  if (sendVerify && user?.email && !user.emailVerified) {
    try {
      await sendEmailVerification(user);
    } catch {
      // Non-fatal; user can request again later.
    }
  }
  reportError(null);
  renderSession(user);
}

async function handleEmail(e) {
  e.preventDefault();
  reportError(null);
  setBusy(true);
  try {
    const email = els.email.value.trim();
    const password = els.password.value;
    const name = els.displayName.value.trim();
    // One path: create if new; if email exists, sign in with the same credentials.
    try {
      const cred = await createUserWithEmailAndPassword(auth, email, password);
      if (name) await updateProfile(cred.user, { displayName: name });
      await afterAuth(cred.user, { sendVerify: true });
    } catch (err) {
      if (err?.code !== "auth/email-already-in-use") throw err;
      const cred = await signInWithEmailAndPassword(auth, email, password);
      await afterAuth(cred.user);
    }
  } catch (err) {
    reportError(err);
  } finally {
    setBusy(false);
  }
}

async function popup(provider) {
  reportError(null);
  setBusy(true);
  try {
    const cred = await signInWithPopup(auth, provider);
    await afterAuth(cred.user);
  } catch (err) {
    reportError(err);
  } finally {
    setBusy(false);
  }
}

function ensureRecaptcha() {
  if (recaptcha) return recaptcha;
  recaptcha = new RecaptchaVerifier(auth, "recaptcha-container", {
    size: "invisible",
  });
  return recaptcha;
}

async function sendSms(e) {
  e.preventDefault();
  reportError(null);
  setBusy(true);
  try {
    const phone = els.phone.value.trim();
    const appVerifier = ensureRecaptcha();
    confirmationResult = await signInWithPhoneNumber(auth, phone, appVerifier);
    els.phoneStep.hidden = true;
    els.smsStep.hidden = false;
    els.smsCode.focus();
  } catch (err) {
    reportError(err);
    try {
      recaptcha?.clear();
    } catch {
      /* ignore */
    }
    recaptcha = null;
  } finally {
    setBusy(false);
  }
}

async function confirmSms(e) {
  e.preventDefault();
  reportError(null);
  setBusy(true);
  try {
    const code = els.smsCode.value.trim();
    const cred = await confirmationResult.confirm(code);
    await afterAuth(cred.user);
  } catch (err) {
    reportError(err);
  } finally {
    setBusy(false);
  }
}

async function doSignOut() {
  reportError(null);
  await signOut(auth);
  confirmationResult = null;
  els.phoneStep.hidden = false;
  els.smsStep.hidden = true;
  els.password.value = "";
  els.smsCode.value = "";
}

function wire() {
  $("form-email")?.addEventListener("submit", handleEmail);
  $("btn-google")?.addEventListener("click", () => popup(new GoogleAuthProvider()));
  $("btn-github")?.addEventListener("click", () => popup(new GithubAuthProvider()));
  $("btn-microsoft")?.addEventListener("click", () => {
    const p = new OAuthProvider("microsoft.com");
    p.setCustomParameters({ prompt: "select_account" });
    return popup(p);
  });
  $("form-phone")?.addEventListener("submit", sendSms);
  $("form-sms")?.addEventListener("submit", confirmSms);
  $("btn-signout")?.addEventListener("click", doSignOut);
  $("btn-resend-verify")?.addEventListener("click", async () => {
    const u = auth.currentUser;
    if (!u?.email) return;
    try {
      await sendEmailVerification(u);
      els.userVerified.textContent = T("acct_verify_sent", "Verification email sent — check your inbox");
    } catch (err) {
      reportError(err);
    }
  });

  onAuthStateChanged(auth, (user) => {
    renderSession(user);
  });

  window.addEventListener("site:i18n", () => {
    renderSession(auth.currentUser);
    if (lastErr) showError(friendlyError(lastErr));
  });
}

wire();
