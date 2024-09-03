use crate::structs::bracket::Bracket;

pub fn generate_base_brackets( team_count: u8, match_id: String ) -> Vec<Bracket>{
  let mut brackets: Vec<Vec<Bracket>> = Vec::new();

  // Add starting set
  let mut set = Vec::new();

  for i in 0..team_count{
    let b = Bracket{ _id: format!("0:{}:{}", i, match_id.clone()), team1: i as i32, team2: -1, winner: i as i32, match_id: match_id.clone() };
    set.push(b);
  }

  brackets.push(set);

  let mut i = 1;
  loop{
    let last_set = &brackets[i - 1];

    let mut set = Vec::new();

    for j in 0..last_set.len().div_ceil(2){
      let j_real = j * 2;

      if j_real + 1 > last_set.len() - 1 {
        set.push(Bracket { _id: format!("{}:{}:{}", i, j, match_id.clone()), team1: j_real as i32, team2: -1, winner: last_set[j_real].winner, match_id: match_id.clone() })
      } else{
        set.push(Bracket { _id: format!("{}:{}:{}", i, j, match_id.clone()), team1: j_real as i32, team2: (j_real + 1) as i32, winner: -1, match_id: match_id.clone() })
      }
    }

    if set.len() == 1{
      brackets.push(set);
      break;
    }

    brackets.push(set);
    i += 1;
  }

  let mut bracket_list = Vec::new();

  for set in brackets{
    for bracket in set{
      bracket_list.push(bracket);
    }
  }

  bracket_list
}