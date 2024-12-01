import { Player } from "./Player";

class Team{
  public _id!: string;
  public name!: string;
  public match_id!: string;
  public players!: Player[];
  public colour!: string;

  public static formatPlayerList( team: Team ): HTMLDivElement{
    let div = document.createElement('div');
    div.classList.add('team-players');

    for (let i = 0; i < team.players.length; i++) {
      if(i === 4){
        div.classList.add('team-players-fade');
        break;
      }

      div.appendChild(<div id={ `player-name-label-${team._id}-${team.players[i]._id}` }>{team.players[i].name}</div> as HTMLElement);
    }

    return div;
  }

  constructor( json: any ){
    this._id = json._id;
    this.match_id = json.match_id;
    this.name = json.name;
    this.players = json.players || [];
  }
}

export { Team }