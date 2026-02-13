// Blinky landing page — minimal JS
// Handles: OS detection, smooth scroll, nav scroll effect, fade-in on scroll

(function () {
  "use strict";

  // --- OS Detection for download button ---
  function detectOS() {
    var ua = navigator.userAgent || "";
    var platform = navigator.platform || "";

    if (/Mac/i.test(platform) || /Mac/i.test(ua)) return "macOS";
    if (/Win/i.test(platform) || /Windows/i.test(ua)) return "Windows";
    if (/Linux/i.test(platform) || /Linux/i.test(ua)) return "Linux";
    return null;
  }

  function updateDownloadButton() {
    var btn = document.getElementById("hero-download");
    if (!btn) return;

    var os = detectOS();
    if (os) {
      btn.textContent = "Download for " + os;
    } else {
      btn.textContent = "Download Blinky";
    }
  }

  // --- Sticky nav background on scroll ---
  function setupNavScroll() {
    var nav = document.getElementById("navbar");
    if (!nav) return;

    function update() {
      if (window.scrollY > 0) {
        nav.classList.add("bg-white/80", "backdrop-blur-md", "border-b", "border-gray-200/60");
      } else {
        nav.classList.remove("bg-white/80", "backdrop-blur-md", "border-b", "border-gray-200/60");
      }
    }

    window.addEventListener("scroll", update, { passive: true });
    update();
  }

  // --- Fade-in on scroll via IntersectionObserver ---
  function setupFadeIn() {
    var elements = document.querySelectorAll(".fade-in");
    if (!elements.length) return;

    if (!("IntersectionObserver" in window)) {
      // Fallback: show everything immediately
      elements.forEach(function (el) {
        el.classList.add("visible");
      });
      return;
    }

    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            entry.target.classList.add("visible");
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.1 }
    );

    elements.forEach(function (el) {
      observer.observe(el);
    });
  }

  // --- Smooth scroll for anchor links ---
  function setupSmoothScroll() {
    document.addEventListener("click", function (e) {
      var link = e.target.closest('a[href^="#"]');
      if (!link) return;

      var targetId = link.getAttribute("href");
      if (targetId === "#") {
        e.preventDefault();
        window.scrollTo({ top: 0, behavior: "smooth" });
        return;
      }

      var target = document.querySelector(targetId);
      if (target) {
        e.preventDefault();
        target.scrollIntoView({ behavior: "smooth" });
      }
    });
  }

  // --- Init ---
  document.addEventListener("DOMContentLoaded", function () {
    updateDownloadButton();
    setupNavScroll();
    setupFadeIn();
    setupSmoothScroll();
  });
})();
