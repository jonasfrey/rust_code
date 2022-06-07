// visit https://www.pixilart.com/draw?ref=home-page
// open browser dev-tools
// paste this script 
// draw char 
// press space 

var b_tog = false;

window.onmouseup = function() {
    b_tog = !b_tog


    if (b_tog) {
        document.querySelector("#pencil-tool").click()
    }
    if (!b_tog) {
        document.querySelector("#eraser-tool").click()
    }

}

document
    .documentElement
    .addEventListener("keydown", function(event) {
        if (event.keyCode === 32) {
            let s_filename = prompt("filename (whitouth .png) ", "test");

            // Do Something, may be an 'Undo' operation
            var s_b64 = document.querySelector("#nav-canvas-img").src;

            var a = document.createElement("a"); //Create <a>
            a.href = s_b64; //Image Base64 Goes here
            a.download = s_filename + ".png"; //File name Here
            a.click(); //Downloaded file
        }
    });