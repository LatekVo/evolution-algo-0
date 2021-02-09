extern crate rand;

use rand::Rng;
       
const RESISTANCE: f64 = 0.98; // 0.98 = 2% resistance
const GRAVITY: f64 = 0.02; // 0.01 unit/time_unit
// stat // const WINGS_COST: f64 = 0.98; // transmission cost/drag
const NODES_COUNT: u32 = 4; // nodes per layer

const RNG_RANGE: f64 = 0.50; //rand -+ percentage
const TIME_STEP: f64 = 0.50; //half time_step for now for time improvment 

#[derive(Clone)]
struct Node {
    input: Vec<f64>,
    input_multiplier: Vec<f64>,

    output: Vec<f64>,
    output_multiplier: Vec<f64>,
}

impl Node {
    fn new(inputs: usize, outputs: usize) -> Self{
    
        //second value doesnt matter for
        //in/out, it matters for multiplier
        
        let mut v_in: Vec<f64> = Vec::new();
        v_in.resize(inputs, 0.5);
        
        let mut v_out: Vec<f64> = Vec::new();
        v_out.resize(outputs, 0.5); 
        
        //why does !vec not work???

        Node {
            input: {
                let mut v: Vec<f64> = Vec::new();
                v.resize(inputs, 0.0);
                v
            },
            input_multiplier: {
                let mut v: Vec<f64> = Vec::new();
                v.resize(inputs, 0.5);
                v 
            },
        
            output: {
                let mut v: Vec<f64> = Vec::new();
                v.resize(outputs, 0.0);
                v
            },
            output_multiplier: {
                let mut v: Vec<f64> = Vec::new();
                v.resize(outputs, 0.5);
                v
            },
        }

    }
}

#[derive(Clone)]
struct Guy {
    //start
    jump: f64,
    power: f64, 

    //stats
    //weight: f64,

    //both wings but one is for glide, 
    //and one if for force it can conert
    wings: f64, //conversion cost
    //glide: f64

    //inputs are volitile
    nodes: Vec<Node>, //node behaviour is non-volitile
    //outputs are volitile
    
    //final
    dist: f64,

    debug: Vec<(String, f64)>
}

/*impl Clone for Guy {
    fn clone(&self) -> Guy {
        *self
    }
}*/

impl Guy {
       
    fn new() -> Self {
        Guy {
            jump: 0.5,
            power: 0.5,

            //weight: 0.5, 

            wings: 0.5, //conversion cost
            //glide: 0.5, //glide

            nodes: {
                
                let mut nodes_tmp: Vec<Node> = Vec::new();
                for _ in 0..NODES_COUNT {
                    //IMPORTANT, INPUTS AND OUTPUTS!
                    //IN
                    //  x_speed
                    //  y_speed
                    //  height (for potential energy)
                    //OUT
                    //  wing angle (conversion of height-speed)
                    nodes_tmp.push(Node::new(3, 2));
                }

                nodes_tmp
            },
            
            debug: Vec::new(),

            dist: 0.0, //decided to not compute it here
        }
    }
    
    fn from(rep: Guy) -> Self {
        // apparently 'self' is same as 'self: Self'
        
        let mut rng_dir:f64 = rand::thread_rng().gen_range(1.0-RNG_RANGE, 1.0+RNG_RANGE);
        let mut rng_wings:f64 = rand::thread_rng().gen_range(1.00-RNG_RANGE, 1.00+RNG_RANGE);
        let mut rng_node = rand::thread_rng(); 
        let mut v_nodes = rep.clone().nodes;
    
        let mut pre_debug: Vec<(String, f64)> = Vec::new();
        

        Guy {

            nodes: {

                for i in 0..v_nodes.len() {
                    for a in 0..v_nodes[i].input_multiplier.len() {
                    v_nodes[i].input_multiplier[a] = 
                        v_nodes[i].input_multiplier[a] * rng_node.gen_range(1.0-RNG_RANGE, 1.0+RNG_RANGE);
                    }
                    for a in 0..v_nodes[i].output_multiplier.len() {
                    v_nodes[i].output_multiplier[a] = 
                        v_nodes[i].output_multiplier[a] * rng_node.gen_range(1.0-RNG_RANGE, 1.0+RNG_RANGE);
                    }
                }

                v_nodes.clone()
            },
            
            jump: {
                if !(0.00 < (rep.jump * rng_dir) && (rep.jump * rng_dir) < 1.00) {
                    rng_dir = 1.00;   
                }
                rep.jump * rng_dir
            },
            power: 1.00 - (rep.jump * rng_dir),
            
            wings: {
                if !(0.00 < (rep.wings * rng_wings) && (rep.wings * rng_wings) < 1.00) {
                    rng_wings = 1.00;   
                }
                rep.wings * rng_wings
            },
            // glide is not used so wings probably will go to top
            // once the incremental bug is fixed
            //glide: 1.00 - (rep.wings * rng_wings),
         
            /* too hard to implement for now,
            weight: {
                if !(0.00 < (rep.weight * rng) && (rep.weight * rng) < 1.00) {
                    rng = 1.00;   
                }
                rep.jump * rng

            }, 
            */

            dist: {
                
                let mut x: f64 = 0.00; 
                let mut y: f64 = 0.00;

                //initial impulse
                let mut y_speed: f64 = rep.jump * rng_dir;
                let mut x_speed: f64 = 1.00 - (rep.jump * rng_dir); //aka rep.power 
      
                let mut wings_angle: f64 = 0.00;
                let mut wing_surface: f64 = 0.00;

                loop { //CALCULATIONS BEGIN

                    //SUPER IMPORTANT!
                    //  outputs min -1 max 1

                    //IMPORTANT, INPUTS AND OUTPUTS!
                    //IN
                    //  x_speed
                    //  y_speed
                    //  height (for potential energy)
                    //OUT
                    //  wing_surface
                    //  wing_angle (conversion of height-speed)
                
                    //setting up nodes
                    for i in 0..v_nodes.clone().len() {
                        
                        v_nodes[i].input[0] = x_speed   * v_nodes[i].input_multiplier[0];
                        v_nodes[i].input[1] = y_speed   * v_nodes[i].input_multiplier[1];
                        v_nodes[i].input[2] = y         * v_nodes[i].input_multiplier[2];
                        //x doesnt matter at all
                
                    }//nodes setup end

                    //node computing begin
                    for i in 0..v_nodes.clone().len() {                                    
                        for j in 0..v_nodes[i].output.clone().len() {
                            for k in 0..v_nodes[i].input.clone().len() {
                            
                                //  input is already multiplied
                                v_nodes[i].output[j] += v_nodes[i].input[k];    
                                //  output is multiplied some lines later

                            } 
                        }                                      
                    }
                    //node computing end
    
                    //assigning node outputs begin
                    // reset before recalculating!
                    wing_surface = 0.5; 
                    wings_angle = 0.5;
                    for i in 0..v_nodes.clone().len() {
                        
                        //Multiplication makes system WAY more controllable, especially
                        //that i didnt design my system to handle '-' values. One more
                        //important note, i SHOULD add, or enable system to procedurally
                        //add '* -1' nodes to easly reverse the value they are handling
                        
                        wing_surface *= v_nodes[i].output[0] * v_nodes[i].output_multiplier[0];
                        wings_angle *=  v_nodes[i].output[1] * v_nodes[i].output_multiplier[1];
                    
                    }
                    if wing_surface > 1.0 {
                        wing_surface = 1.0;   
                    }
                    if wing_surface < 0.0 {
                        wing_surface = 0.0;
                    }
                    
                    if wings_angle > 1.0 {
                        wings_angle = 1.0;   
                    }
                    if wings_angle < 0.0 {
                        wings_angle = 0.0;
                    }//assigning node outputs end
                    
                    let old_x_speed = x_speed; //DEBUG
                    let old_y_speed = y_speed; //DEBUG

                    //                      Conversion*efficiency*controll*direction
                    //                efficiency                 angle           efficiency2      this is really funky
                    let lift = (rep.wings * rng_wings) * (wings_angle-0.5)*2.0 * wing_surface * x_speed*0.5 * y_speed*0.5 * RESISTANCE; 
                    //                                 number center correction
                        //Yes, resistance is double-calculated
                        //because lift creates additional drag
                                                                                                            

                    //[ 0 min | 1 max ] Ughhh, finding correct formula is so tidious
                    x_speed = (x_speed - lift) * RESISTANCE;
                    //                 | reverse        
                    y_speed = (y_speed + lift) * RESISTANCE - GRAVITY; 
                    //decided to add gravity here for it to impact resistance calculations with delay
                    //                    may do RESISTANCE * RESISTANCE;
                    
                    println!("   .   x_gain: {}", x_speed - old_x_speed); //a bit of unnecessary inefficiency but                     
                    println!("   .   y_gain: {}", y_speed - old_y_speed); //this code is a big pile of mess anyways

                    x += x_speed * TIME_STEP; //smaller time_step% may lead to more precision
                    y += y_speed * TIME_STEP;
                    println!("      x: {}     y: {}", x, y);
                    if (y < 0.00) || (x_speed < 0.00) {
                        if x_speed < 0.00 {
                            pre_debug.push((String::from("finished prematurly due to reverse speed: "), x_speed));
                        }
                        pre_debug.push((String::from("wings_angle"), wings_angle));
                        pre_debug.push((String::from("wing_surface"), wing_surface));
                        pre_debug.push((String::from("wing:glide"), rep.wings * rng_wings));
                        pre_debug.push((String::from("lift"), lift));
                        break;
                    }  
                } //END CALCULATIONS
                
                println!("  current candidate's dist: {} pwr: {}%, jmp: {}%", 
                    x, rep.power * rng_dir * 100.0, rep.jump * rng_dir * 100.0);

                x //returned
                
            },

            debug: pre_debug,
        }
    }
}

fn main() {

    let mut guys: Vec<Guy> = Vec::new();
    let mut lap_scores: Vec<Guy> = Vec::new();

    let mut check = 0;
    let mut last_best_score = 0.00;
    let mut best_guy: Guy;
     
    best_guy = Guy::new();

    loop {
        
        for _ in 0..1_000 {  
            guys.push(Guy::from(best_guy.clone()));
        }

        //'i' is 'Guy'! remember this
        for guy in &guys { 
            if guy.dist > last_best_score {
                best_guy = guy.clone();
            }
        }

        lap_scores.push(best_guy.clone());

        while guys.len() > 0 {
            guys.remove(0);
        }
        
        println!("Current top candidate's forward power: {}% and jump power: {}%", 
            best_guy.power * 100.00, best_guy.jump * 100.00);

        if best_guy.clone().dist == last_best_score {check += 1};
        if check == 10 { break };
        last_best_score = best_guy.clone().dist;
    }
    
    //i know, but i had to do this atrocity
    println!("Laps: {}", lap_scores.len());
    for guy in 0..lap_scores.len() {
        println!("Lap {} top contender's score: {}, jmp: {}%, pwr: {}%", 
            guy, lap_scores[guy].dist, 
            lap_scores[guy].jump * 100.00, 
            lap_scores[guy].power * 100.00);
    }
    println!("And the best result is {} achived with jmp at {}% and pwr at {}%", 
        last_best_score, best_guy.clone().jump * 100.00, best_guy.clone().power * 100.00);
    
    println!("  using this {} node positions:", best_guy.nodes.clone().len());
    for i in 0..best_guy.nodes.clone().len() {
        println!("      Node{} inputs | outputs", i);
        let mut b = 0; //for output counter
        for a in 0..best_guy.nodes[i].input_multiplier.clone().len() { 
            print!("              {}        ", best_guy.clone().nodes[i].input_multiplier[a]);
            
            if b < best_guy.nodes[i].output_multiplier.clone().len() {
                print!("{}", best_guy.clone().nodes[i].output_multiplier[b]); 
                b += 1;
            }
            println!();
        }
    }

    
    println!("and individual stats of last keyframe:");
    for i in 0..best_guy.debug.clone().len() {
        let (a, b) = best_guy.debug[i].clone();
        println!("  {}: {}", a, b);
    }
}
