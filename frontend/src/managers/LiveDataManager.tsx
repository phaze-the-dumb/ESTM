import { Match } from '../structs/Match';
import * as cooki from './CookiManager';

// Add an element to the global "window" variable to store the cache
declare global{
  interface Window{
    LiveDataManager: LiveDataManager
  }
}

class LiveDataManager{
  private _ws: WebSocket;
  private _sentHello = false;

  private _teamsSocketUpdate: ( msg: any ) => void = () => {};

  private constructor(){
    this._ws = new WebSocket(window.ENDPOINT + '/api/v1/live');

    this._ws.onopen = () => {
      console.log('Connected to backend');

      if(cooki.getStore('token'))
        this.sendHello();
    }

    this._ws.onmessage = ( e ) => {
      let json = JSON.parse(e.data);
      console.log(json);

      switch(json.type){
        case 'select-match':
          window.MatchManager.selectMatchDisplay(json.match._id);
          break;
        case 'rename-match':
          window.MatchManager.renameDisplay(json.match._id, json.match.name);
          break;
        case 'delete-match':
          window.MatchManager.deleteDisplay(json.match._id);
          if(window.BracketDiagramManager)window.BracketDiagramManager.fetchData(window.MatchManager.selected());

          break;
        case 'create-match':
          let match = new Match(json.match._id, json.match.name);
          window.MatchManager.addMatch(match);
          break;
        case 'create-team':
          this._teamsSocketUpdate(json);
          if(window.BracketDiagramManager)window.BracketDiagramManager.fetchData(window.MatchManager.selected());

          break;
        case 'rename-team':
          this._teamsSocketUpdate(json);
          if(window.BracketDiagramManager)window.BracketDiagramManager.fetchData(window.MatchManager.selected());

          break;
        case 'delete-team':
          this._teamsSocketUpdate(json);
          if(window.BracketDiagramManager)window.BracketDiagramManager.fetchData(window.MatchManager.selected());

          break;
        case 'start-match':
          window.MatchManager.startMatch();
          break;
        case 'cancel-match':
          window.MatchManager.cancelMatch();
          break;
        case 'next-bracket':
          window.MatchManager.setNextTeam1And2(json.bracket.team1, json.bracket.team2);
          break;
        case 'current-bracket':
          window.MatchManager.setTeam1And2(json.bracket.team1, json.bracket.team2);
          break;
        case 'win-bracket':
          window.MatchManager.setWinningTeam(json.team);
          break;
      }
    }
  }

  public teamSocketUpdate( cb: ( msg: any ) => void ){
    this._teamsSocketUpdate = cb;
  }

  public sendHello(){
    if(this._sentHello)return;
    this._sentHello = true;

    this._ws.send(JSON.stringify({ type: 'auth', token: cooki.getStore('token') }));
  }

  public static Init(){
    window.LiveDataManager = new LiveDataManager(); // When "Init" is called create a new instance of LiveDataManager and fill in "window.LiveDataManager"
  }
}

export { LiveDataManager } // Export LiveDataManager so other scripts can use the type / call static functions