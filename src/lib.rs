extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod gl_setup;
mod shaders;
mod programs;
mod common_funcs;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;
mod app_state;
mod constants;

//#[wasm_bindgen] makes things in js see rust code
#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct Client{
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
    program_color_2d_gradient: programs::Color2DGradient,
    program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl Client{

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self{
        console_error_panic_hook::set_once();
        //gets graphics layer from gl_setup interface
        let gl = gl_setup::initialize_webgl_context().unwrap();
        
        Self{
            program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            program_color_2d: programs::Color2D::new(&gl),
            program_graph_3d: programs::Graph3D::new(&gl),
            gl: gl,
        }
    }

    //underscore next to var removes unused warnings
    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        //dimensions of rendered box
        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,//bottom
            curr_state.control_top,//top
            curr_state.control_left,//left
            curr_state.control_right,//right
            curr_state.canvas_height,//canvas_height
            curr_state.canvas_width,//canvas_width
        );

       

        self.program_graph_3d.render(
            &self.gl,
            curr_state.control_bottom,//bottom
            curr_state.control_top,//top
            curr_state.control_left,//left
            curr_state.control_right,//right
            curr_state.canvas_height,//canvas_height
            curr_state.canvas_width,//canvas_width
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            &common_funcs::get_updated_3d_y_values(curr_state.time),
        );
            
        
    }
}