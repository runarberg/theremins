function audioContext() {
  "use strict";

  var self = {};
  var AudioContext = window.AudioContext || window.webkitAudioContext;
  var audioCtx = new AudioContext();

  var sineWaves = [];

  Object.defineProperty(self, "data", { value: data });
  function data(arr) {
    var update = sineWaves.slice(0, arr.length);
    var exit = sineWaves.slice(arr.length);
    var enter = arr.slice(sineWaves.length, arr.length)
          .map(function(d) { return new SineWave(d); });

    update.forEach(function(sine, i) { sine.update(arr[i]); });
    enter.forEach(function(sine) { sineWaves.push(sine); });
    exit.forEach(function(sine) { sine.stop(); });
    sineWaves.splice(arr.length, exit.length);
  }

  return Object.freeze(self);

  function SineWave(descriptor) {
    var oscillator = audioCtx.createOscillator();
    var gainNode = audioCtx.createGain();

    oscillator.type = "sine";
    oscillator.frequency.value = descriptor && descriptor.frequency || 440;
    gainNode.gain.value = descriptor && descriptor.volume || 1;

    oscillator.connect(gainNode);
    gainNode.connect(audioCtx.destination);

    Object.defineProperty(this, "update", {
      value: function(d) {
        oscillator.frequency.value = d.frequency;
        gainNode.gain.value = d.volume;
      }
    });

    Object.defineProperty(this, "stop", {
      value: function() {
        oscillator.stop();
        gainNode.disconnect(audioCtx.destination);
      }
    });

    oscillator.start();
    return Object.freeze(this);
  };
}
