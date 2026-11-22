(function () {
  var path = (location.pathname || "/").replace(/\/+$/, "") || "/";
  // Only highlight a chip that exists in #topnav. Support and data-deletion
  // live in the legal footer, not the primary chips — do not mark About/Privacy.
  var map = {
    "/": "home",
    "/about": "about",
    "/privacy": "privacy",
    "/terms": "terms",
    "/cancellation": "cancellation",
    "/account": "account"
  };
  var key = map[path];
  if (!key) return;
  var links = document.querySelectorAll("#topnav a[data-nav]");
  for (var i = 0; i < links.length; i++) {
    if (links[i].getAttribute("data-nav") === key) {
      links[i].classList.add("active");
      links[i].setAttribute("aria-current", "page");
    }
  }
})();
