// Firebase web app (Analytics) for wadhu.com — bundled into static/js/firebase-init.js.
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";

const firebaseConfig = {
  apiKey: "AIzaSyAD-K5O2xyDY8kQ7mb_xbqFHhlDv8mn4BY",
  authDomain: "wadhu-ac8dc.firebaseapp.com",
  projectId: "wadhu-ac8dc",
  storageBucket: "wadhu-ac8dc.firebasestorage.app",
  messagingSenderId: "349667335886",
  appId: "1:349667335886:web:4d1947e47f223159cafa9f",
  measurementId: "G-PFL833HNZ4",
};

const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);

export { app, analytics };
