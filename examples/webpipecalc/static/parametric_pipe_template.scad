// The thickness of the material
thickness                = 4;

// The inner width (mouth width) of the pipe
width                    = 47.3428;

// The inner depth of the pipe
depth                    = 41.8602;

// The resonator length (you might want to add a few centimeters and increase the tuning slot length)
length                   = 589.8759;

// The thickness of the jet gap
jet_thickness            = 1.2702;

// The cutup height of the pipe (distance between jet and upper lip)
cutup_height             = 16.57;

// The angle of the upper labium (lip)
labium_angle             = 5;

// The initial thickness of the upper labium - can be required for 3D printing in some special cases. Otherwise set this to 0.
labium_initial_thickness = 0;

// The length of the foot of the pipe
foot_length              = 40;

// The diameter of the hole for the air supply
hole_diameter            = 15.6113;

// The length of the final piece of the jet
jet_length               = 5;

// If true, generate a stop for the pipe, if false, add a tuning slot
stopped_pipe             = false;

// The length of the tuning slot
tuning_slot_length       = length * 0.15;

// The width of the tuning slot
tuning_slot_width        = width * 0.5;

// The gap around the tuning stop
stop_gap                 = 1;

// The thickness of the tuning stop
stop_thickness           = 10;

// The length of the handle of the tuning stop
stop_handle_length       = 20;

// The diameter of the handle of the tuning stop
stop_handle_diameter    = width / 4;

// If true, place the pipe and its lid next to each other to allow for easier 3D printing
print_model              = true;

// ===== END OF PARAMETERS =====

module labial_pipe_lids(thickness,width,length,labium_angle,labium_initial_thickness,cutup_height,foot_length,stopped_pipe,tuning_slot_length,tuning_slot_width) {
    x_offset = width / 2 + thickness;
    lid_length = length - cutup_height;
    front_back_width = width + 2 * thickness;
    upper_lip_height = thickness / tan(labium_angle);
    
    // foot front
    translate([-x_offset,-thickness,-foot_length]) cube([front_back_width,thickness,foot_length]);
    
    // upper lip
    difference() {
        translate([0,0,cutup_height])
            rotate([90,-90,90])
                linear_extrude(height = front_back_width, center = true)
                    polygon([[0,0],[lid_length,0],[lid_length,thickness],[upper_lip_height,thickness],[0,labium_initial_thickness]]);
        if (!stopped_pipe) {
            translate([-tuning_slot_width/2,-1.5*thickness,length-tuning_slot_length]) cube([tuning_slot_width,2 * thickness,2*tuning_slot_length]);
        }
    }
}

module labial_pipe_stop(thickness,width,depth,length,stop_gap,stop_thickness,stop_handle_length,stop_handle_diameter) {
    stop_width = width - 2 * stop_gap;
    stop_depth = depth - 2 * stop_gap;
    translate([0,0,length - stop_thickness]) union () {
        translate([-stop_width/2,stop_gap,0]) cube([stop_width,stop_depth,stop_thickness]);
        translate([0,depth/2,stop_thickness]) cylinder(stop_handle_length * 0.9,d = stop_handle_diameter,$fn = 50);
        translate([0,depth/2,stop_thickness + stop_handle_length - stop_handle_diameter/2]) sphere(d = 1.3*stop_handle_diameter,$fn = 50);
    }
}

module labial_pipe(thickness,width,depth,length,jet_thickness,foot_length,hole_diameter,jet_length) {
    x_offset = width / 2 + thickness;
    total_length = foot_length + length;
    front_back_width = width + 2 * thickness;
    core_depth = depth - jet_thickness;
    core_length = foot_length - thickness;
    
    // right wall
    translate([x_offset - thickness,0,-foot_length]) cube([thickness,depth,total_length]);
    
    // left wall
    translate([-x_offset,0,-foot_length]) cube([thickness,depth,total_length]);
    
    // back wall
    translate([-x_offset,depth,-foot_length]) cube([front_back_width,thickness,total_length]);

    // foot underpart with hole
    difference() {
        translate([-x_offset+thickness,0,-foot_length]) cube([width,depth,thickness]);
        translate([0,depth/2,-foot_length+thickness/2]) cylinder(h = 1.2 * thickness, d = hole_diameter, center = true, $fn = 50);
    }
    
    // core
    translate([0,depth,-core_length]) rotate([90,-90,90]) linear_extrude(height = width, center = true) polygon([[0,0],[core_length,0],[core_length,core_depth],[core_length-jet_length,core_depth]]);
}

union() {
    if (print_model) {
        translate([width / 2 + thickness,foot_length,depth+thickness]) rotate([-90,0,0])
        labial_pipe(thickness,width,depth,length,jet_thickness,foot_length,hole_diameter,jet_length);
    } else {
        labial_pipe(thickness,width,depth,length,jet_thickness,foot_length,hole_diameter,jet_length);
    }
    
    if (print_model) {
        translate([width / 2 + width + 5 * thickness,foot_length,0]) rotate([-90,0,0])
        labial_pipe_lids(thickness,width,length,labium_angle,labium_initial_thickness,cutup_height,foot_length,stopped_pipe,tuning_slot_length,tuning_slot_width);
    } else {
        labial_pipe_lids(thickness,width,length,labium_angle,labium_initial_thickness,cutup_height,foot_length,stopped_pipe,tuning_slot_length,tuning_slot_width);
    }
    
    if (stopped_pipe) {
        if (print_model) {
            translate([width / 2 + 8 * thickness + 2 * width,0,-length+stop_thickness]) labial_pipe_stop(thickness,width,depth,length,stop_gap,stop_thickness,stop_handle_length,stop_handle_diameter);
        } else {
            labial_pipe_stop(thickness,width,depth,length,stop_gap,stop_thickness,stop_handle_length,stop_handle_diameter);
        }
    }
}
