// Firebase web app for wadhu.com — Analytics + Auth.
// Bundled into static/js/firebase-init.js and reused by account.js.
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import { getAuth } from "firebase/auth";

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

export { app, auth, analytics };
