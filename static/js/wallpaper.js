const displayText = `
╔─────────────────────────────────────────────────────────────────────╗
│                                                                     │
│  ███████████            ████                                        │
│ ░█░░░███░░░█           ░░███                                        │
│ ░   ░███  ░  █████ ████ ░███   ██████  ████████                     │
│     ░███    ░░███ ░███  ░███  ███░░███░░███░░███                    │
│     ░███     ░███ ░███  ░███ ░███████  ░███ ░░░                     │
│     ░███     ░███ ░███  ░███ ░███░░░   ░███                         │
│     █████    ░░███████  █████░░██████  █████                        │
│    ░░░░░      ░░░░░███ ░░░░░  ░░░░░░  ░░░░░                         │
│               ███ ░███                                              │
│              ░░██████                                               │
│               ░░░░░░                                                │
│  █████                                                              │
│ ░░███                                                               │
│  ░███         ██████  █████ ███ █████ ████████   ██████  █████ ████ │
│  ░███        ███░░███░░███ ░███░░███ ░░███░░███ ███░░███░░███ ░███  │
│  ░███       ░███ ░███ ░███ ░███ ░███  ░███ ░░░ ░███████  ░███ ░███  │
│  ░███      █░███ ░███ ░░███████████   ░███     ░███░░░   ░███ ░███  │
│  ███████████░░██████   ░░████░████    █████    ░░██████  ░░███████  │
│ ░░░░░░░░░░░  ░░░░░░     ░░░░ ░░░░    ░░░░░      ░░░░░░    ░░░░░███  │
│                                                           ███ ░███  │
│                                                          ░░██████   │
│                                                           ░░░░░░    │
│                                                                     │
╚─────────────────────────────────────────────────────────────────────╝
`

let logoY = 10;
let logoX = 10;
let xSpeed = 2;
let ySpeed = 2;

function draw() {
  const canvas = document.getElementById("wallpaper");
  const canvasContext = canvas.getContext("2d");

  const scale = window.devicePixelRatio
  canvas.width = Math.floor(window.innerWidth * scale);
  canvas.height = Math.floor(window.innerHeight * scale);
  canvasContext.scale(scale, scale)
  drawLogo();
  setInterval(drawLogo, 10);

}

function drawLogo() {

  const canvas = document.getElementById("wallpaper");
  const canvasContext = canvas.getContext("2d");
  canvasContext.clearRect(0, 0, canvas.width, canvas.height);
  canvasContext.font = "10px/10px \"EnvyCodeR Nerd Font Mono\"";
  canvasContext.fillStyle = "white";

  var lines = displayText.split('\n');

  var yIncrement = 10;
  var xIncrement = 10;


  logoX += yIncrement;
  logoY += xIncrement;

  var y = 0;
  for (var i = 0; i < lines.length; ++i) {
    canvasContext.fillText(lines[i], logoX, logoY + y);
    y += 10;
  }
}

function drawLogo() {
  const canvas = document.getElementById("wallpaper");
  const canvasContext = canvas.getContext("2d");
  canvasContext.clearRect(0, 0, canvas.width, canvas.height);
  canvasContext.font = "10px/10px \"EnvyCodeR Nerd Font Mono\"";
  canvasContext.fillStyle = "white";

  var lines = displayText.split('\n');
  var lineHeight = 10;

  // Calculate the width and height of the text
  let textWidth = 0;
  for (let i = 0; i < lines.length; i++) {
    let lineWidth = canvasContext.measureText(lines[i]).width;
    if (lineWidth > textWidth) {
      textWidth = lineWidth;
    }
  }
  let textHeight = lines.length * lineHeight;

  // Bounce off the edges
  if (logoX + textWidth > window.innerWidth || logoX < 0) {
    xSpeed = -xSpeed;
  }
  if (logoY + textHeight > window.innerHeight || logoY < 0) {
    ySpeed = -ySpeed;
  }

  logoX += xSpeed;
  logoY += ySpeed;

  // Draw the text
  let y = 0;
  for (let i = 0; i < lines.length; i++) {
    canvasContext.fillText(lines[i], logoX, logoY + y);
    y += lineHeight;
  }
}

window.addEventListener("load", draw);
