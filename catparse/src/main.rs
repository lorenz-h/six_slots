use std::result::Result;

pub struct CategoricalMask{
  pub kind: String,
  pub legal_values: Vec<String>,
}

#[derive(Debug)]
pub struct CategorialSlotValue{
  kind: String,
  value: String,
}

impl CategoricalMask {
  pub fn new(designated_kind: &String) -> Result<CategoricalMask, String > {
    let mut legal_vals: Vec<String> = vec![];

    if designated_kind == "Room"{
      legal_vals.push("kitchen".to_string());
      legal_vals.push("bathroom".to_string());
    } else {
      return Err(String::from("pter"));
    }
    return Ok(CategoricalMask {kind:designated_kind.to_string(), legal_values: legal_vals});
  }
  pub fn parse(&self, statement: &String, ) -> Vec<CategorialSlotValue>{

    let mut occurrences = vec![];

    for val in &self.legal_values{
      if statement.contains(&val[..]){
        occurrences.push(CategorialSlotValue {kind: self.kind.clone(), value:val.clone()})
      }
    }
    println!("found occurrence(s) {:?}", occurrences );
    return occurrences;
  }
}

fn main(){
  let cp = CategoricalMask::new(&"Room".to_string()).unwrap();
  cp.parse(&String::from("turn on the light in kitchen and the bathroom"));
}