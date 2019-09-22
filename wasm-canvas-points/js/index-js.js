import createFps from 'fps-indicator'

var canvas = document.getElementById('canvas');
var ctx = canvas.getContext('2d');

function getRndColor() {
    var r = 255*Math.random()|0,
        g = 255*Math.random()|0,
        b = 255*Math.random()|0,
        a = Math.random();
    return 'rgba(' + r + ',' + g + ',' + b + ',' + a + ')';
}

const renderLoop = () => {
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);
    for (var i=0; i<2000; i++) {
        ctx.beginPath();
        ctx.arc(Math.random() * w, Math.random() * h, Math.random() * 25, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.fillStyle = getRndColor();
        ctx.fill();
    }

    requestAnimationFrame(renderLoop);
};

renderLoop();
