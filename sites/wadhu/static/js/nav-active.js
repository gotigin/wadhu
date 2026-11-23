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
  var links = document.querySelectorAll("#topnav a[data-nav]");
  if (key) {
    for (var i = 0; i < links.length; i++) {
      if (links[i].getAttribute("data-nav") === key) {
        links[i].classList.add("active");
        links[i].setAttribute("aria-current", "page");
      }
    }
  }

  var bar = document.getElementById("topbar");
  var toggle = document.getElementById("nav-toggle");
  var nav = document.getElementById("topnav");
  if (!bar || !toggle || !nav) return;

  function setOpen(open) {
    bar.classList.toggle("nav-open", open);
    toggle.setAttribute("aria-expanded", open ? "true" : "false");
    toggle.textContent = open ? "Close" : "Menu";
  }

  toggle.addEventListener("click", function () {
    setOpen(!bar.classList.contains("nav-open"));
  });

  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") setOpen(false);
  });
})();
