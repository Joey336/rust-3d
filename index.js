const rust = import('./pkg/rust_3d');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', {antialias: true});

rust.then(m => {
    if (!gl){
        alert('Failed to initialize WebGl');
        return;
    }



    //Create Cleint struct
    const Client = new m.Client();
    //get time at program start
    const initialTime = Date.now();

    const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
    var lastDrawTime = -1; //in milliseconds

    function render(){
        window.webkitRequestAnimationFrame(render);
        const currTime = Date.now();

        if(currTime >= lastDrawTime + FPS_THROTTLE){
            lastDrawTime = currTime;
            
            //if browser if resized, resets width/height/style values
            if(window.innerHeight != canvas.height || window.innerWidth != canvas.width){
                canvas.height = window.innerHeight;
                canvas.width = window.innerWidth;
                canvas.style.height = window.innerHeight;
                
                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currTime - initialTime;
            Client.update(elapsedTime, window.innerHeight, window.innerWidth);
            Client.render();
            //Rust Update Call

            //Rust Render Call
        }
    }


    render();
});