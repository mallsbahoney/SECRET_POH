
let ws = null;
let canvas = document.getElementById("frameCanvas");
let ctx = canvas.getContext("2d");
let log = document.getElementById("log");

function startStream(mode) {
  if (ws) ws.close();
  ws = new WebSocket("ws://localhost:9001");
  ws.onopen = () => ws.send(mode);
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (mode === "json") {
      log.textContent = JSON.stringify(data, null, 2);
    } else if (data.type === "frame") {
      let img = ctx.createImageData(data.width, data.height);
      for (let i = 0; i < data.pixels.length; i++) {
        const v = data.pixels[i];
        img.data[i * 4 + 0] = v;
        img.data[i * 4 + 1] = v;
        img.data[i * 4 + 2] = v;
        img.data[i * 4 + 3] = 255;
      }
      ctx.putImageData(img, 0, 0);
    }
  };
}

function submitTask() {
  if (!ws) return;
  const inputText = document.getElementById("taskInput").value;
  const fileInput = document.getElementById("fileInput");

  try {
    const json = JSON.parse(inputText);

    // If file selected, base64 encode and attach
    if (fileInput.files.length > 0) {
      const file = fileInput.files[0];
      const reader = new FileReader();
      reader.onload = () => {
        const base64 = btoa(reader.result);
        json.filename = file.name;
        json.filedata = base64;
        ws.send(JSON.stringify(json));
      };
      reader.readAsBinaryString(file);
    } else {
      ws.send(JSON.stringify(json));
    }
  } catch (e) {
    alert("Invalid JSON");
  }
}
