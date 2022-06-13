
#[allow(unused_macros)]
macro_rules! log {
    //($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    ($($t:tt)*) => (gloo_console::log!(format!($($t)*)))
}

pub fn rescale<T>(actions: &mut Vec<(T,Option<f32>)>) {
    let mut min = 1.1;
    let mut max = -0.1;
    let mut avg = 0.0;
    let mut cnt = 1.0;

    for (_,f) in actions.iter() {
        if let Some(f) = *f {
            if min > f {
                min = f;
            }
            
            if max < f {
                max = f;
            }
            
            avg += f;
            cnt += 1.0;
        }
    }
    
    avg /= cnt;
    
    let max_scale = max - avg;
    let min_scale = avg - min;
    
    let scale = 
        if max_scale > min_scale {
            max_scale
        } else {
            min_scale
        };
    
    for (_,f) in actions.iter_mut() {
        if let Some(fraction) = f {
            *f = Some((*fraction - avg)/scale);
        }
    }
}
