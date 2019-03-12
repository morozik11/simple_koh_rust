extern crate rand;
extern crate rustc_serialize;

use self::rand::Rng;
use koh;


pub fn get_average_vectors(vectors: &Vec<Vec<f32>>,vectors_for: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    
    let mut list_summ = vec![];
    
    let new = vectors_for.clone();
    
    for vector in vectors {
            
        list_summ.push(summ(vector));
            
    }
    
    let max:f32;
    let min:f32;
    
    if list_summ[0] > list_summ[1]{
            
        max = list_summ[0];
        min = list_summ[1];
            
    } else {
		
	max = list_summ[1];
	min = list_summ[0];
		
    }
	
    let average = (max + min) / 2.0;
    
    let max_average = (average + max) / 2.0;
    let min_average = (average + min) / 2.0;
    
    let mut new_ = vec![];
    
    for vector in new {
            
        let summ_ = summ(&vector);
            
        if summ_ <= max_average && summ_ >= min_average {
                    
            new_.push(vector);
                    
        } 
            
    }
    
    new_
	
}

pub fn get_max_min_vectors(vectors: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
	
    let mut list_summ = vec![];
    
    let new = vectors.clone();
    
    for vector in vectors {
            
        list_summ.push(summ(vector));
            
    }
    
    let min_key = min_elem_key(&list_summ);
    let max_key = max_elem_key(&list_summ); 
    
    let mut new_ = vec![];
    
    let mut i = 0;
    
    for n in new{
            
            if i == min_key || i == max_key {
                    new_.push(n);
            }
            
            i = i + 1;
            
    }
    
    new_
	
}

pub fn min_elem_key(vector: &Vec<f32>) -> usize{
    
    let mut smallest = vector[0];
    let mut i = 0;
    let mut key = 0;
    
    for elem in vector{
        if *elem < smallest { 
            smallest = *elem;
            key = i;
        }
        i = i + 1;
    } 

    key

}

pub fn  max_elem_key(vector: &Vec<f32>) -> usize{
    
    let mut largest = vector[0];
    let mut i = 0;
    let mut key = 0;
    
    for elem in vector{
        if *elem > largest {
            largest = *elem;
            key = i;
        }
        i = i + 1;
    }

    key

}

pub fn summ(vector: &Vec<f32>)->f32{
	
    let mut sum = 0.0;
    
    for elem in vector{
            
            sum = *elem + sum;
            
    }
    
    sum
	
}

pub fn get_max_min(vectors: &Vec<Vec<f32>>) -> (f32,f32) {
    
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

pub fn min_elem(vector: &Vec<f32>) -> f32{
    
    let mut smallest = vector[0];
    
    for elem in vector{
        if *elem < smallest { 
            smallest = *elem;
        }
    } 

    smallest

}

pub fn max_elem(vector: &Vec<f32>) -> f32{
    
    let mut largest = vector[0];
    
    for elem in vector{
        if *elem > largest {
            largest = *elem;
        }
    }

    largest

}

pub fn get_a_b(max_min: &(f32,f32)) -> (f32,f32){
    
    let a = 1.0 / (max_min.0 - max_min.1);
    let mut b = max_min.1 * (-1.0);
    b = b / (max_min.0 - max_min.1);
    
    (a,b)

}



pub fn normalize(vector: &Vec<f32>, a_b: &(f32,f32)) -> Vec<f32> {
    
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

pub fn wrap_normalize(vectors: &Vec<Vec<f32>>, a_b: &(f32,f32)) -> Vec<Vec<f32>>{
    
    let mut normalized = vec![];
    
    for vector in vectors {
        
        let vector_ = normalize(vector,a_b);
        normalized.push(vector_);
        
    }
    
    normalized
    
}

pub fn generate_rand_vs(leng_vs: usize, leng_v: usize) -> Vec<Vec<f32>>{
    
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

pub fn multiplication(v_x: &mut Vec<f32>, x: f32){
    
    for el in v_x {
        
        *el = *el * x;
        
    }
    
}

pub fn addition_vectors(v_x: &Vec<f32>,v_y: &mut Vec<f32>){
    
    let mut i = 0;
    
    let mut new_v = vec![];
    
    for el in v_x {
        
        let new_el = *el + v_y[i];
        new_v.push(new_el);
        i = i + 1;
    }
    
    *v_y = new_v;
    
}

pub fn difference_vectors(v_x: &Vec<f32>,v_y: &Vec<f32>) -> Vec<f32>{
    
    let mut i = 0;
    
    let mut new_v = vec![];
    
    for el in v_x {
        
        let mut new_el = *el - v_y[i];
        new_v.push(new_el);
        
        i = i + 1;
    }
    
    new_v
    
}

pub fn search_min(store: &Vec<(f32,usize)>) -> (f32,usize){
    
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

pub fn classification(vs_trained: &mut Vec<Vec<f32>>, vs: &mut Vec<Vec<f32>>) -> Vec<ClassVector>{
    
    let clone = vs.clone();
    
    let mut i = 0;
    let leng = vs_trained.len();
    let mut store = vec![];
    
    let mut result = vec![];
    
    for v in clone {
        
        while i < leng{
            
            let sum = euclidean(&vs_trained[i],&v);
            store.push((sum,i));
            i = i + 1;
            
        }
        
        let mut smallest = search_min(&store);
        let obj = ClassVector { vector : v,class : smallest.1 };
        
        result.push(obj);
        
        store = vec![];
        i = 0;
        
    }
    
    result
    
}

pub fn wrap_training(vs: &mut Vec<Vec<f32>>, vs_rand: &mut Vec<Vec<f32>>){
    
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

pub fn training_vectors(vs: &mut Vec<Vec<f32>>, vs_rand: &mut Vec<Vec<f32>>, lambda: f32){
    
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

pub fn euclidean(vec_x: & Vec<f32>, vec_y: &Vec<f32>) -> f32{
    
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
