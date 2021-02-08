extern crate rand;

use rand::Rng;
       
const RESISTANCE: f64 = 0.98; // 0.98 = 2% resistance
const GRAVITY: f64 = 0.02; // 0.01 unit/time_unit
// stat // const WINGS_COST: f64 = 0.98; // transmission cost/drag
const NODES_COUNT: u32 = 4; // nodes per layer

const RNG_RANGE: f64 = 0.20; //rand -+ percentage

const TIME_STEP: f64 = 1.00;

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
    glide: f64, //glide

    //inputs are volitile
    nodes: Vec<Node>, //node behaviour is non-volitile
    //outputs are volitile
    
    //final
    dist: f64
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
            glide: 0.5, //glide

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

            dist: 0.0, //decided to not compute it here
        }
    }
    
    fn from(rep: Guy) -> Self {
        // apparently 'self' is same as 'self: Self'
        
        let mut rng_dir:f64 = rand::thread_rng().gen_range(1.0-RNG_RANGE, 1.0+RNG_RANGE);
        let mut rng_wings:f64 = rand::thread_rng().gen_range(1.00-RNG_RANGE, 1.00+RNG_RANGE);
        let mut rng_node = rand::thread_rng(); 
        let mut v_nodes = rep.clone().nodes;

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
            glide: 1.00 - (rep.wings * rng_wings),
         
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

                loop {

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
                
                    for i in 0..v_nodes.clone().len() {
                        
                        v_nodes[i].input[0] = x_speed * v_nodes[i].input_multiplier[0];
                        v_nodes[i].input[1] = y_speed * v_nodes[i].input[1];
                        v_nodes[i].input[2] = y * v_nodes[i].input[2];
                        //x doesnt matter at all
                
                    } 
                    //computing begin
                    

                    
                    //computing end

                    for i in 0..v_nodes.clone().len() {
                        
                        wing_surface += v_nodes[i].output[0] * v_nodes[i].output_multiplier[0];
                        wings_angle += v_nodes[i].output[1] * v_nodes[i].output_multiplier[0];
                    
                    }
                    if wing_surface > 1.0 {
                        wing_surface = 1.0;   
                    }
                    if wing_surface < 0.0 {
                        wing_surface = 0.0;
                    }
                    
                    if wing_surface > 1.0 {
                        wing_surface = 1.0;   
                    }
                    if wing_surface < -1.0 {
                        wing_surface = -1.0;
                    }


                    //[ 1 up | 0 middle | -1 down ]
                    x_speed = (x_speed - ((x_speed * (rep.wings * rng_wings) * wing_surface) * wings_angle)) * RESISTANCE;
                    //                 ^ reverse v |    Conversion*efficiency*controll*direction
                    y_speed = (y_speed - GRAVITY)+((x_speed * (rep.wings * rng_wings) * wings_angle * wing_surface)) * RESISTANCE;
                    //                                                            may do RESISTANCE * RESISTANCE;
                    
                    x += x_speed*TIME_STEP;
                    y += y_speed*TIME_STEP;
                    if y < 0.00 {
                        break;
                    } 
                }
                
                println!("  current candidate's dist: {} pwr: {}%, jmp: {}%", 
                    x, rep.power * rng_dir * 100.0, rep.jump * rng_dir * 100.0);

                x //returned
                
            }, 
            //i should replace this with calculations
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
    println!("  using this node positions:");
    println!("  Node{} inputs | outputs", 0);
    println!("          {}        {}", best_guy.nodes[0].input_multiplier[0], best_guy.nodes[0].output_multiplier[0]); 
    println!("          {}        {}", best_guy.nodes[0].input_multiplier[1], best_guy.nodes[0].output_multiplier[1]); 
    println!("          {}          ", best_guy.nodes[0].input_multiplier[2]);
    println!("          {}          ", best_guy.nodes[0].input_multiplier[3]);
    println!("  Node{} inputs | outputs", 1);
    println!("          {}        {}", best_guy.nodes[1].input_multiplier[0], best_guy.nodes[1].output_multiplier[0]); 
    println!("          {}        {}", best_guy.nodes[1].input_multiplier[1], best_guy.nodes[1].output_multiplier[1]); 
    println!("          {}          ", best_guy.nodes[1].input_multiplier[2]);
    println!("          {}          ", best_guy.nodes[1].input_multiplier[3]);
    println!("  Node{} inputs | outputs", 2);
    println!("          {}        {}", best_guy.nodes[2].input_multiplier[0], best_guy.nodes[2].output_multiplier[0]); 
    println!("          {}        {}", best_guy.nodes[2].input_multiplier[1], best_guy.nodes[2].output_multiplier[1]); 
    println!("          {}          ", best_guy.nodes[2].input_multiplier[2]); 
    println!("          {}          ", best_guy.nodes[2].input_multiplier[3]); 
   /* println!("  Node{} inputs | outputs", 3);
    println!("          {}        {}", best_guy.nodes[3].input_multiplier[0], best_guy.nodes[3].output_multiplier[0]); 
    println!("          {}        {}", best_guy.nodes[3].input_multiplier[1], best_guy.nodes[3].output_multiplier[1]); 
    println!("          {}          ", best_guy.nodes[3].input_multiplier[2]); 
    println!("          {}          ", best_guy.nodes[3].input_multiplier[3]); 
*/
}
