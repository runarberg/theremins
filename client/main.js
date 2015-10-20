"use strict";

var τ = 2 * Math.PI;

var ws = new WebSocket("ws://192.168.2.101:3012");
ws.onopen = app;

function app() {
  var canvas = byId("canvas");
  var controls = byId("controls");
  var minFrequency = controls.querySelector("input[name='min-frequency']");
  var maxFrequency = controls.querySelector("input[name='max-frequency']");

  window.addEventListener("resize", function() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight - controls.clientHeight;
  });

  var sineWaves = [];
  ws.onmessage = function(msg) {
    sineWaves = JSON.parse(msg.data);
  };

  var x = function(t) {
    var log2_a = Math.log2(+minFrequency.value);
    var log2_a_m_b = Math.log2(+maxFrequency.value) - log2_a;
    return !isNaN(log2_a) && !isNaN(log2_a_m_b) ?
      Math.pow(2, log2_a + t * log2_a_m_b) :
      0;
  };

  var x_inv = function(t) {
    var log2_a = Math.log2(+minFrequency.value);
    var log2_a_m_b = Math.log2(+maxFrequency.value) - log2_a;
    return  !isNaN(log2_a) && !isNaN(log2_a_m_b) ?
      (Math.log2(t) - log2_a) / log2_a_m_b :
      0;
  };
  var y = function(t) { return 1 - t; };
  var y_inv = y;

  var mouse = {
    x: 0,
    y: 0,
    down: false
  };

  var color = "rgb(" + [0, 0, 0].map(function() {
    return (Math.random() * 255)|0;
  }).join(",") + ")";

  canvas.addEventListener("mousedown", function() { mouse.down = true; });
  canvas.addEventListener("mouseup", function() { mouse.down = false; });
  canvas.addEventListener("mousemove", function(e) {
    mouse.x = e.clientX / canvas.width;
    mouse.y = e.clientY / canvas.height;
  });

  draw();
  play();


  function play() {
    var audio = audioContext();

    updateAudio();

    function updateAudio() {
      ws.send(JSON.stringify({
        mute: !mouse.down,
        frequency: x(mouse.x),
        volume: y(mouse.y),
        color: color
      }));

      audio.data(sineWaves);
      window.requestAnimationFrame(updateAudio);
    }
  }

  function draw() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight - controls.clientHeight;
    var ctx = canvas.getContext('2d');

    ctx.fillStyle = "green";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    updateCanvas();

    function updateCanvas() {
      ctx.fillStyle = "green";
      ctx.globalAlpha = 1;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      var hertz = 27.5;
      do {
        linspace(12, hertz, 2 * hertz).forEach(markLine("#33AA33"));
        markLine("white", hertz);
        hertz *= 2;
      } while (hertz < +maxFrequency.value);

      sineWaves.forEach(function(d) {
        if (!d.mute) {
          markTone(d);
        }
      });

      window.setTimeout(function() {
       window.requestAnimationFrame(updateCanvas); 
      }, 60);
    }

    function markLine(color, frequency) {
      if (typeof frequency === "undefined") {
        return function(freq) { markLine(color, freq); };
      }
      var lineX = x_inv(frequency) * canvas.width;
      ctx.save();
      ctx.strokeStyle = color;
      ctx.globalAlpha = 1;
      ctx.beginPath();
      ctx.moveTo(lineX, 0);
      ctx.lineTo(lineX, canvas.height);
      ctx.stroke();
      return ctx.restore();
    };

    function markTone(sine) {
      var x = x_inv(sine.frequency);
      var y = y_inv(sine.volume);
      ctx.save();
      ctx.fillStyle = "white";
      ctx.fillStyle = sine.color;
      ctx.strokeStyle = "black";
      ctx.globalAlpha = 1;
      ctx.beginPath();
      ctx.arc(x * canvas.width, y * canvas.height, 15, 0, τ);
      ctx.stroke();
      ctx.fill();
      return ctx.restore();
    };
  }
}

function linspace(n, a, b) {
  var arr = new Float64Array(n);
  var mean = (b - a) / n;
  for (var i = 0; i < n; i += 1) {
    arr[i] = a + i * mean;
  }
  return arr;
}

function byId(id) { return document.getElementById(id); }
