
struct FilterCondition<T>{
    filter : T,
    operator: char, //yeni eklediğimiz alan
}


impl<T: PartialOrd> FilterCondition <T> {
    fn is_match(&self,item: &T) -> bool {
        match self.operator {
            '>' => item > &self.filter,
            '<' => item < &self.filter,
            '=' => item == &self.filter,
            _ => false //geçersiz operatör durumunda false döndürür
        }
    }
}


fn custom_filter<T>(list: Vec<T> , condition: &FilterCondition<T>) -> Vec<T> where T: PartialOrd {
    return list.into_iter().filter(|item: &T| condition.is_match(item)).collect();
}

fn main(){

    let numbers = vec![7.0,14.0,10.0,15.0,20.0];
    let condition = FilterCondition{filter: 14.0, operator: '='}; //eşitlik kontrolü yapacak koşul

    let filtered_list = custom_filter(numbers, &condition);
    println!("{:?}" , filtered_list); //sadece 14.0 olan elemanları içeren liste
}
