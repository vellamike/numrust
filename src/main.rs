fn add_arrays(arry1 : &Vec<int>, arry2 : &Vec<int>) -> Vec<int>{ 
    let mut sum_arry = Vec::new();

    for i in range(0,arry1.len()) {
        sum_arry.push(arry1[i] + arry2[i])    
    }
    return sum_arry;
}

fn dot_product(arry1 : &Vec<int>, arry2 : &Vec<int>) -> int {
    
    let mut j = 0;

    for i in range(0,arry1.len()) {
        j += arry1[i] * arry2[i]    
    }

    return j;
}


#[deriving(Show)]
struct Array {
    internal_vector: Vec<int>
}

impl Add<Array, Array> for Array{
    fn add(self, _rhs: Array)  -> Array {

    let d = Array {
        internal_vector : add_arrays(&self.internal_vector,&_rhs.internal_vector)
    };

    return d;
    }
}

fn main() {
    //Make two vectors
    let a : Vec<int> = vec![1i,2i,3i, 5i];
    let c : Vec<int> = vec![2i,3i,4i, 6i];
    let d = add_arrays(&a,&c);
    println!("{}", d);
    let dot_product = dot_product(&a,&c);
    println!("{}", dot_product);

    let j = Array {
        internal_vector : vec![1i,2i]
    };

    let d = Array {
        internal_vector : vec![3i,4i]
    };

    let x : Array = j + d;

    println!("{}", x)
}
