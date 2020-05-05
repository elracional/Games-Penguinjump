//Funcion que toma el textarea y va agragando las funciones que se ejecutan
function addCode(text1, text2) {
  var code = ""; //Inicializa variable
  if(arguments.length === 1) { //Si solo tiene un argumento
    code = text1 + "();";
  } else {
    code = text1 + text2 + "();";
  }
  document.getElementById("code").value += code + "\n"; //Agrega el texto al textarea
  document.getElementById("code").scrollTop = document.getElementById("code").scrollHeight; //El scroll del textarea se va hasta abajo
}
