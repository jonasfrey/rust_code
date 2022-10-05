var canvas = document.createElement("canvas");
var ctx = canvas.getContext("2d");

var myImg = new Image();
myImg.onload = function() {
   ctx.drawImage(myImg, 0, 0);
};
var s_url = "https://cdn.openai.com/dall-e-2/demos/text2im/astronaut/horse/photo/7.jpg"
myImg.src = s_url;

document.body.appendChild(canvas)