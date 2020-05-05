//Carga de audios en variables
var audio = new Audio();
audio.src = "./media/dreaming.ogg";
audio.loop = true;

var jump = new Audio();
jump.src = "./media/jump.mp3";

var fish = new Audio();
fish.src = "./media/eat.mp3";

var life = new Audio();
life.src = "./media/life.mp3";

var down = new Audio();
down.src = "./media/slide.mp3";

//Funciones de reproducción a llamar desde el código
function playMusic() {
  audio.currentTime = 0;
  audio.play();
}

function stopMusic() {
  audio.pause();
}

function playJump() {
  jump.currentTime = 0;
  jump.play();
}

function playFish() {
  fish.currentTime = 0;
  fish.play();
}

function playLife() {
  fish.currentTime = 0;
  life.play();
}

function playDown() {
  down.currentTime = 0;
  down.play();
}
