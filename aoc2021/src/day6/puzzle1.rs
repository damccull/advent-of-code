use super::Fish;

pub fn count_fish_in_pond(pond: Vec<Fish>) -> usize {
    let mut pond = pond;
    //dbg!(&pond);
    for _day in 0..80 {
        let fishies = &pond.clone();
        for (i, fish) in fishies.iter().enumerate() {
            if fish.age == 0 {
                pond[i].age = 7;
                pond.push(Fish { age: 8 });
            }
            pond[i].age -= 1;
        }
    }
    pond.len()
}
