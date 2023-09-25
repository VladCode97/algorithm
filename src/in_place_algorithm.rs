
pub fn change_position(data: &mut Vec<Vec<u32>>, reverse_data: Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    for i in 0..data.len()  {
        for j in 0..data.len() {
            data[i][j] = reverse_data[j][i];
        }
    }
    return data.clone();
}

pub fn reverse_array(data: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut reverse_data: Vec<Vec<u32>> = Vec::new();
    for datum in data.iter().rev() {
        reverse_data.push(datum.clone());
    }
    return change_position(data, reverse_data);
}


