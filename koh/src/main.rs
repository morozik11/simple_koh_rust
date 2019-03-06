extern crate rand;

use rand::Rng;


fn main() {

    let mut training0 = vec![1222.6,2200.90,2223.9,1200.2];
    let mut training1 = vec![114.5,167.7,150.3,100.4];
    let mut training2 = vec![22.5,33.7,44.3,55.4];
    let mut training3 = vec![1.5,1.7,2.3,22.4];
    
    let mut vs = vec![training0,training1,training2,training3];
    
    let mut v0 = vec![1.6,1.9,1.9,1.2];
    let mut v1 = vec![114.5,167.7,150.3,100.4];
    let mut v2 = vec![1.5,2.7,1.3,3.4];
    let mut v3 = vec![34.5,65.7,32.3,47.4];
    let mut v4 = vec![1222.5,2200.7,2223.3,1200.4];
    let mut v5 = vec![101.5,177.7,157.3,103.4];
    let mut v6 = vec![1000.5,2000.7,1000.3,1000.4];
    let mut v7 = vec![1222.5,2200.7,2223.3,1200.4];
    let mut v8 = vec![2.5,1.7,2.3,1.4];
    let mut v9 = vec![12.5,10.7,12.3,14.4];
    let mut v10 = vec![114.5,167.7,150.3,100.4];
    let mut v11 = vec![1141.5,1677.7,1500.3,1000.4];
    
    let mut vs_ = vec![v0,v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11];
    
    /* training */
    let max_min = common(&vs);
    let a_b = get_a_b(&max_min);
    wrap_normalize(&mut vs, &a_b);

    let mut new_vs = generate_rand_vs(vs.len(),vs[0].len());
    wrap_training(&mut vs, &mut new_vs);
    /* training */
    
    
    /*classification*/
    let max_min_ = common(&vs_);
    let a_b_ = get_a_b(&max_min_);
    wrap_normalize(&mut vs_, &a_b_);
    classification(&mut new_vs, &mut vs_);
    /*classification*/
    
}

fn common(vectors: &Vec<Vec<f32>>) -> (f32,f32) {
    
    let mut max = max_elem(&vectors[0]);
    let mut min = min_elem(&vectors[0]);
    let mut max_;
    let mut min_;
    
    for vector in vectors{
        
        max_ = max_elem(vector);
        min_ = min_elem(vector);
        
        if max_ > max{
            max = max_;
        }

        if min_ < min{
            min = min_;
        }

    }
    
    (max,min)
    
}

fn min_elem(vector: &Vec<f32>) -> f32{
    
    let mut smallest = vector[0];
    
    for elem in vector{
        if *elem < smallest { 
            smallest = *elem;
        }
    } 

    smallest

}

fn  max_elem(vector: &Vec<f32>) -> f32{
    
    let mut largest = vector[0];
    
    for elem in vector{
        if *elem > largest {
            largest = *elem;
        }
    }

    largest

}

fn get_a_b(max_min: &(f32,f32)) -> (f32,f32){
    
    let a = 1.0 / (max_min.0 - max_min.1);
    let mut b = max_min.1 * (-1.0);
    b = b / (max_min.0 - max_min.1);
    
    (a,b)

}



fn normalize(vector: &Vec<f32>, a_b: &(f32,f32)) -> Vec<f32> {
    
    let mut new_vector = vec![];

    for i in vector {
        
        let mut el = (a_b.0 * *i) + a_b.1;
        if el < 0.0 {
            el = 0.0;
        }

        new_vector.push(el);
        
    }

    new_vector
    
}

fn wrap_normalize(vectors: &mut Vec<Vec<f32>>, a_b: &(f32,f32)){
    
    for vector in vectors {
        
        let vector_ = normalize(vector,a_b);
        *vector = vector_;
        
    }
    
}

fn generate_rand_vs(leng_vs: usize, leng_v: usize) -> Vec<Vec<f32>>{
    
    let mut new_vs = vec![];
    let mut  i = 0;

    while i < leng_vs { 

        let mut new_v = vec![];
        let mut iter = 0;

        while iter < leng_v {
            new_v.push(rand::thread_rng().gen_range(0.1, 0.3));
            iter = iter+1;
        }
        
        new_vs.push(new_v);
        i = i +1;

    }

    new_vs

}

fn multiplication(v_x: &mut Vec<f32>, x: f32){
    
    for el in v_x {
        
        *el = *el * x;
        
    }
    
}

fn addition_vectors(v_x: &Vec<f32>,v_y: &mut Vec<f32>){
    
    let mut i = 0;
    
    let mut new_v = vec![];
    
    for el in v_x {
        
        let new_el = *el + v_y[i];
        new_v.push(new_el);
        i = i + 1;
    }
    
    *v_y = new_v;
    
}

fn difference_vectors(v_x: &Vec<f32>,v_y: &Vec<f32>) -> Vec<f32>{
    
    let mut i = 0;
    
    let mut new_v = vec![];
    
    for el in v_x {
        
        let mut new_el = *el - v_y[i];
        new_v.push(new_el);
        
        i = i + 1;
    }
    
    new_v
    
}

fn search_min(store: &Vec<(f32,usize)>) -> (f32,usize){
    
    let mut smallest = store[0].0;
    let mut smallest_result = store[0];
    
    for s in store {
        
        if s.0 < smallest  {
            
            smallest = s.0;
            smallest_result = *s;
        
        }
        
    }
    
    smallest_result
    
}

fn classification(vs_trained: &mut Vec<Vec<f32>>, vs: &mut Vec<Vec<f32>>){
    
    let mut i = 0;
    let leng = vs_trained.len();
    let mut store = vec![];
    
    for v in vs {
        
        while i < leng{
            
            let sum = euclidean(&vs_trained[i],v);
            store.push((sum,i));
            i = i + 1;
            
        }
        
        let mut smallest = search_min(&store);
        
        println!("{:?}",v);
        println!("{:?}",vs_trained[smallest.1]);
        println!("{:?}",smallest.1);
        println!("==============================================================");
        
        store = vec![];
        i = 0;
        
    }
    
}

fn wrap_training(vs: &mut Vec<Vec<f32>>, vs_rand: &mut Vec<Vec<f32>>){
    
    let mut lambda = 0.3;
    let delta = 0.05;
    
    while lambda > 0.0 {
        
        let mut i = 10;
        
        while i > 0 {
            
            training_vectors(vs,vs_rand,lambda);
            
            i = i -1;
        }
        
        lambda = lambda - delta;
    }
    
}

fn training_vectors(vs: &mut Vec<Vec<f32>>, vs_rand: &mut Vec<Vec<f32>>, lambda: f32){
    
    let mut i = 0;
    let leng = vs_rand.len();
    
    let mut store = vec![];
    
    for v in vs {
        
        while i < leng{
            
            let sum = euclidean(&vs_rand[i],v);
            store.push((sum,i));
            i = i + 1;
            
        }
        
        let mut smallest = search_min(&store);
        
        let mut diff = difference_vectors(v,&vs_rand[smallest.1]);
        multiplication(&mut diff, lambda);
        addition_vectors(&diff,&mut vs_rand[smallest.1]); 
        
        i = 0;
        store = vec![];
        
    }
    
}

fn euclidean(vec_x: & Vec<f32>, vec_y: &Vec<f32>) -> f32{
    
    let mut i = 0;

    let mut sum  = 0.0;

    for el in vec_x{

        let mut mov = *el - vec_y[i];
        mov = mov*mov;
        sum = sum+mov;
        i = i+1;

    }

    sum.sqrt()

}
