use macroquad::prelude::*;
use dualshock_driver::DualShock4;

#[macroquad::main("3D")]
async fn main() {
    let mut vo = ViewObj::new();

    loop {
        vo.view_task().await;
        vo.read_task().await;
    }
}


pub struct ViewObj
{
    ds:DualShock4,
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub c_x:f32,
    pub c_y:f32,
    pub c_z:f32
}

impl ViewObj {
    pub fn new()->ViewObj
    {
        let ds = DualShock4::new().unwrap();

        ViewObj
        {
            ds:ds,
            x:0.0,
            y:0.0,
            z:0.0,
            c_x:-20.0,
            c_y:15.0,
            c_z:0.0
        }
    }
    pub async fn read_task(&mut self)
    {
        let _ = self.ds.read().await;
        self.x += self.ds.sticks.left_y*0.1;
        self.y += self.ds.sticks.right_y*0.1;
        self.z += self.ds.sticks.left_x*0.1;

        if self.ds.dpad.up_key
        {
            self.c_y += 0.1;
        }
        else if self.ds.dpad.down_key
        {
            self.c_y -= 0.1;
        }

        if self.ds.dpad.right_key
        {
            self.c_z += 0.1;
        }
        else if self.ds.dpad.left_key
        {
            self.c_z -= 0.1;
        }

        if self.ds.btns.triangle
        {
            self.c_x += 0.1
        }
        else if self.ds.btns.cross
        {
            self.c_z -= 0.1
        }
    }
    pub async fn view_task(&mut self)
    {
            clear_background(LIGHTGRAY);
                    
            set_camera(&Camera3D {
                position: vec3(self.c_x, self.c_y, self.c_z),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                ..Default::default()
            });

            draw_grid(20, 1., BLACK, GRAY);


            // rl
            draw_cube_wires(vec3(self.x*0.1, self.y*0.1, 0.0), vec3(1., 1., 1.), YELLOW);

            // rr
            draw_cube_wires(vec3((self.x+self.robo_size)*0.1, self.robot.target_position.y*0.1, 0.0), vec3(1., 1., 1.), YELLOW);

            // fr
            draw_cube_wires(vec3((self.robot.target_position.x+self.robo_size)*0.1, (self.robot.target_position.y+self.robo_size)*0.1, 0.0), vec3(1., 1., 1.), YELLOW);

            // fl
            draw_cube_wires(vec3((self.robot.target_position.x)*0.1, (self.robot.target_position.y+self.robo_size)*0.1, 0.0), vec3(1., 1., 1.), YELLOW);

            set_default_camera();
            draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

            next_frame().await
    }
}