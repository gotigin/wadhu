/* Android Chrome Desktop-site / pinch-zoom guard for Makepad sites.
 * Hardens the viewport meta and clamps html/body width before wasm lays out.
 * Height is left to Makepad (URL-bar show/hide must not thrash body height).
 * Must stay aligned with scripts/web/mobile_layout_size.js.
 *
 * Resize is debounced and style writes are skipped when the width is unchanged.
 * Undebounced window.resize writes fought Makepad layout and froze DevTools
 * responsive mode (2026-11-23). */
(function () {
  try {
    var meta = document.querySelector('meta[name="viewport"]');
    var content =
      "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover";
    if (meta) meta.setAttribute("content", content);
    else {
      meta = document.createElement("meta");
      meta.name = "viewport";
      meta.content = content;
      document.head.appendChild(meta);
    }
    var touch = "ontouchstart" in window || (navigator.maxTouchPoints || 0) > 0;
    if (!touch) return;
    var timer = null;
    var lastW = 0;
    function clampNow() {
      var ws = [window.innerWidth, window.outerWidth, screen.width].filter(function (x) {
        return x > 0;
      });
      if (!ws.length) return;
      var w = Math.min.apply(null, ws);
      w = Math.max(1, Math.round(w));
      if (w === lastW) return;
      lastW = w;
      document.documentElement.style.width = w + "px";
      document.documentElement.style.maxWidth = w + "px";
      if (document.body) {
        document.body.style.width = w + "px";
        document.body.style.maxWidth = w + "px";
      }
    }
    function clampDebounced() {
      if (timer !== null) clearTimeout(timer);
      timer = setTimeout(function () {
        timer = null;
        clampNow();
      }, 150);
    }
    clampNow();
    window.addEventListener("resize", clampDebounced);
    window.addEventListener("orientationchange", clampDebounced);
    if (window.visualViewport) {
      window.visualViewport.addEventListener("resize", clampDebounced);
    }
  } catch (_e) {}
})();
