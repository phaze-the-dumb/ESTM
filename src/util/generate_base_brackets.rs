use crate::structs::bracket::Bracket;

pub fn generate_base_brackets( team_count: u8, match_id: String ) -> Vec<Bracket>{
  let mut brackets: Vec<Vec<Bracket>> = Vec::new();

  // Add starting set
  let mut set = Vec::new();

  for i in 0..team_count{
    // Fill the first set with half brackets, that each identify a team
    let b = Bracket{ _id: format!("0:{}:{}", i, match_id.clone()), team1: i as i32, team2: -1, winner: i as i32, match_id: match_id.clone() };
    set.push(b);
  }

  brackets.push(set);

  // Loop through all the sets
  let mut i = 1;
  loop{
    // Get the previous set of brackets
    let last_set = &brackets[i - 1];

    let mut set = Vec::new();

    for j in 0..last_set.len().div_ceil(2){ // Loop through all the last brackets and sort them
      let j_real = j * 2;

      if j_real + 1 > last_set.len() - 1 {
        set.push(Bracket { _id: format!("{}:{}:{}", i, j, match_id.clone()), team1: j_real as i32, team2: -1, winner: last_set[j_real].winner, match_id: match_id.clone() })
        // If we cannot find another team to pair this team with we will use -1 to signify there is no other team
      } else{
        set.push(Bracket { _id: format!("{}:{}:{}", i, j, match_id.clone()), team1: j_real as i32, team2: (j_real + 1) as i32, winner: -1, match_id: match_id.clone() })
      }
    }

    if set.len() == 1{ // If they is only one bracket ( the final ) left then break out of the loop
      brackets.push(set); // Add the newest set into the list of sets
      break;
    }

    brackets.push(set); // Add the newest set into the list of sets
    i += 1;
  }

  let mut bracket_list = Vec::new(); // Convert the 2d array into a single 1d array so the database can process it

  for set in brackets{
    for bracket in set{
      bracket_list.push(bracket);
    }
  }

  bracket_list // Return the 1d list of brackets
}