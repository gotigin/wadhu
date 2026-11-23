// Firebase web app for wadhu.com — Analytics + Auth.
// Bundled into static/js/firebase-init.js and reused by account.js.
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import {
  getAuth,
  GoogleAuthProvider,
  signInWithPopup,
  signInWithEmailAndPassword,
  createUserWithEmailAndPassword,
} from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyAD-K5O2xyDY8kQ7mb_xbqFHhlDv8mn4BY",
  // Custom domain so OAuth returns via https://wadhu.com/__/auth/handler
  authDomain: "wadhu.com",
  projectId: "wadhu-ac8dc",
  storageBucket: "wadhu-ac8dc.firebasestorage.app",
  messagingSenderId: "349667335886",
  appId: "1:349667335886:web:4d1947e47f223159cafa9f",
  measurementId: "G-PFL833HNZ4",
};

const app = initializeApp(firebaseConfig);
const auth = getAuth(app);

let analytics = null;
try {
  analytics = getAnalytics(app);
} catch {
  // Analytics may fail in some local/offline contexts; Auth still works.
}

if (typeof window !== "undefined") {
  window.__SITE_AUTH__ = auth;
  window.__SITE_APP__ = app;
  window.__SITE_PRODUCT_NAME__ = window.__SITE_PRODUCT_NAME__ || "Wadhu";
  window.__SITE_ACCOUNT_HREF__ = window.__SITE_ACCOUNT_HREF__ || "/account";
  window.__SITE_SIGN_IN_GOOGLE__ = () =>
    signInWithPopup(auth, new GoogleAuthProvider());
  window.__SITE_SIGN_IN_EMAIL__ = (email, password) =>
    signInWithEmailAndPassword(auth, email, password);
  window.__SITE_REGISTER_EMAIL__ = (email, password) =>
    createUserWithEmailAndPassword(auth, email, password);
}

export { app, auth, analytics };
