const fractal_descriptions = [
  {
    name: "dragon",
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_dragon(canvas, iterations);
    }
  },
  {
    name: "terdragon",
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_terdragon(canvas, iterations);
    }
  }
];

/**********************************************************
 * Config
 **********************************************************/

function set_visible_config(selected_fractal) {
  let config_panels = document.querySelectorAll(".config");
  for (panel of config_panels) {
    if (panel.id === selected_fractal + "-config") {
      panel.style.display = "block";
    } else {
      panel.style.display = "none";
    }
  }
}

set_visible_config(
  document.querySelector("#fractal-type").selectedOptions[0].value
);

/**********************************************************
 * Load the wasm
 **********************************************************/
import("../pkg/fractal_wasm")
  .then(fractal => {
    document.fractal = fractal;
    let canvas = document.querySelector("#fractal-canvas");

    // Show coordinates within the canvas
    canvas.addEventListener("pointermove", event => {
      document.querySelector("#coords").innerText =
        "Canvas coords: X: " + event.clientX + ", Y: " + event.clientY;

      let othercoords = fractal.screen_to_turtle(
        canvas,
        event.clientX,
        event.clientY
      );
      document.querySelector("#turtle-coords").innerText =
        "Turtle coords: X: " + othercoords[0] + ", Y: " + othercoords[1];
    });

    let fractal_picker = document.querySelector("#fractal-type");
    fractal_picker.addEventListener("input", event => {
      let choice = event.target.selectedOptions[0];
      let selected_fractal = choice.value;

      set_visible_config(selected_fractal);
    });

    for (desc of fractal_descriptions) {
      document
        .querySelector("#" + desc.name + "-iterations")
        .addEventListener("input", desc.run_config(canvas, fractal));
    }
  })
  .catch(console.error);
