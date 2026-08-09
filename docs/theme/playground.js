// Browser shape playground glue (RFC-093). Deliberately thin: it only reads
// input boxes, calls the wasm-bindgen exports, and writes the returned
// string into the page. All shape and error logic lives in Rust
// (tools/matten-playground/src/lib.rs), where it is unit tested.
//
// Loaded via `additional-js` on every page (mdBook has no per-page hook), so
// the very first check is whether the playground's markup is even present.
(function () {
  "use strict";

  var container = document.getElementById("shape-playground");
  if (!container) {
    return;
  }

  var statusEl = document.getElementById("pg-status");
  function setStatus(text) {
    if (statusEl) {
      statusEl.textContent = text;
    }
  }

  function val(id) {
    var el = document.getElementById(id);
    return el ? el.value : "";
  }

  function showResult(outputId, text) {
    var el = document.getElementById(outputId);
    if (el) {
      el.textContent = text;
    }
  }

  function wireButton(buttonId, run) {
    var button = document.getElementById(buttonId);
    if (button) {
      button.addEventListener("click", run);
    }
  }

  setStatus("Loading the WebAssembly module…");

  // Resolved relative to this script's own URL, so it works regardless of
  // the page's depth in the book (mdBook already root-relativizes this
  // script's own <script src>; this keeps the wasm-bindgen module lookup
  // consistent with that, rather than assuming a fixed site path).
  var moduleURL = new URL(
    "../playground/matten_playground.js",
    document.currentScript.src
  );

  import(moduleURL.href)
    .then(function (mod) {
      return mod.default().then(function () {
        setStatus("");

        wireButton("pg-broadcast-run", function () {
          showResult(
            "pg-broadcast-output",
            mod.playground_broadcast(
              val("pg-broadcast-left-shape"),
              val("pg-broadcast-left-values"),
              val("pg-broadcast-right-shape"),
              val("pg-broadcast-right-values")
            )
          );
        });

        wireButton("pg-reshape-run", function () {
          showResult(
            "pg-reshape-output",
            mod.playground_reshape(
              val("pg-reshape-shape"),
              val("pg-reshape-values"),
              val("pg-reshape-target")
            )
          );
        });

        wireButton("pg-axis-run", function () {
          showResult(
            "pg-axis-output",
            mod.playground_axis_reduce(
              val("pg-axis-shape"),
              val("pg-axis-values"),
              val("pg-axis-axis"),
              val("pg-axis-op")
            )
          );
        });

        wireButton("pg-matmul-run", function () {
          showResult(
            "pg-matmul-output",
            mod.playground_matmul(
              val("pg-matmul-left-shape"),
              val("pg-matmul-left-values"),
              val("pg-matmul-right-shape"),
              val("pg-matmul-right-values")
            )
          );
        });

        wireButton("pg-numeric-run", function () {
          showResult(
            "pg-numeric-output",
            mod.playground_try_numeric(
              val("pg-numeric-shape"),
              val("pg-numeric-values")
            )
          );
        });
      });
    })
    .catch(function (err) {
      setStatus(
        "The playground's WebAssembly module failed to load (" +
          err +
          "). A plain local `mdbook build` does not build it — see the " +
          "note on this page for the command that does."
      );
    });
})();
