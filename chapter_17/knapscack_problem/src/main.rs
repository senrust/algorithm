#[derive(Clone, PartialEq)]
enum Select {
    UnSelect,
    Select,
}
struct Object {
    value: usize,
    weight: usize,
}

fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let object_num = input_info[0] + 1;
    let max_weight = input_info[1];
    let mut object_info: Vec<Object> = vec![];
    let mut max_value: Vec<Vec<usize>> = vec![vec![0; max_weight + 1]; object_num];
    let mut select_object: Vec<Vec<Select>> = vec![vec![Select:: UnSelect; max_weight + 1]; object_num]; 

    for _ in 0..input_info[0] {
        let input_object: Vec<usize> = input::read_numline();
        let object = Object{value: input_object[0], weight: input_object[1]};
        object_info.push(object);
    }

    for object_index in 1..object_num {
        let object = &object_info[object_index - 1];
        for weight in 1..=max_weight {
            if weight < object.weight {
                max_value[object_index][weight] = max_value[object_index - 1][weight] ;
                continue;
            } 
            if max_value[object_index - 1][weight] < max_value[object_index - 1][weight - object.weight] + object.value {
                max_value[object_index][weight] = max_value[object_index - 1][weight - object.weight] + object.value;
                select_object[object_index][weight] = Select:: Select;
            } else {
                max_value[object_index][weight] = max_value[object_index - 1][weight] ;
            }
        }
    }
    let mut serachindex = input_info[0];
    let mut search_weight = max_weight;

    println!("max value is {}", max_value[object_num -1][max_weight]);

    while serachindex >= 1 {
        if select_object[serachindex][search_weight] == Select:: Select {
            println!("object {} is selected", serachindex -1);
            search_weight -= object_info[serachindex -1].weight;
        }
        serachindex -= 1;
    }
}
