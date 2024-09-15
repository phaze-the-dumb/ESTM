import { createSignal } from "solid-js";
import { Match } from "../structs/Match";
import * as cooki from './CookiManager';
import { Accessor, Setter } from "solid-js";
import { Team } from "../structs/Team";

// Add an element to the global "window" variable to store the matches
declare global{
  interface Window{
    MatchManager: MatchManager
  }
}

class MatchManager{
  private _selectedMatchData: Match | null = null;

  private _matches: Match[] = [];
  private _matchList: any = {};

  private _matchStatusText: HTMLDivElement;
  private _matchHTMLList: HTMLDivElement | null = null;

  private _bracketsHookCB: (( match: Match | null ) => void ) | null = null;
  private _matchChangeCB: (( match: Match | null ) => void ) | null = null;

  private _matchesChangeCB: (( match: Match[], selected: Match | null ) => void )[] = [];

  private _hasFetchedData = false;

  private _setIsPlaying: Setter<boolean>;
  private _setBracketWinner: Setter<number>;

  private _setPlayingNextTeam1: Setter<Team | null>;
  private _setPlayingNextTeam2: Setter<Team | null>;

  private _setPlayingTeam1: Setter<Team | null>;
  private _setPlayingTeam2: Setter<Team | null>;

  public loaded = false;

  public isPlaying: Accessor<boolean>;
  public bracketWinner: Accessor<number>;

  public playingNextTeam1: Accessor<Team | null>;
  public playingNextTeam2: Accessor<Team | null>;

  public playingTeam1: Accessor<Team | null>;
  public playingTeam2: Accessor<Team | null>;

  private constructor(
    matchStatusText: HTMLDivElement
  ){
    this._matchStatusText = matchStatusText;

    let [ isPlaying, setIsPlaying ] = createSignal(false);

    this.isPlaying = isPlaying;
    this._setIsPlaying = setIsPlaying;

    let [ bracketWinner, setBracketWinner ] = createSignal(0);

    this.bracketWinner = bracketWinner;
    this._setBracketWinner = setBracketWinner;

    let [ playingNextTeam1, setPlayingNextTeam1 ] = createSignal<Team | null>(null);
    let [ playingNextTeam2, setPlayingNextTeam2 ] = createSignal<Team | null>(null);

    this.playingNextTeam1 = playingNextTeam1;
    this._setPlayingNextTeam1 = setPlayingNextTeam1;

    this.playingNextTeam2 = playingNextTeam2;
    this._setPlayingNextTeam2 = setPlayingNextTeam2;

    let [ playingTeam1, setPlayingTeam1 ] = createSignal<Team | null>(null);
    let [ playingTeam2, setPlayingTeam2 ] = createSignal<Team | null>(null);

    this.playingTeam1 = playingTeam1;
    this._setPlayingTeam1 = setPlayingTeam1;

    this.playingTeam2 = playingTeam2;
    this._setPlayingTeam2 = setPlayingTeam2;
  }

  public startMatch(){
    this._setIsPlaying(true);
  }

  public cancelMatch(){
    this._setIsPlaying(false);
  }

  public setNextTeam1And2( team1: Team, team2: Team){
    this._setPlayingNextTeam1(team1);
    this._setPlayingNextTeam2(team2);
  }

  public setTeam1And2( team1: Team, team2: Team){
    this._setPlayingTeam1(team1);
    this._setPlayingTeam2(team2);

    this._setBracketWinner(0);
  }

  public setWinningTeam( team: string ){
    switch(team){
      case 'team1':
        this._setBracketWinner(1);
        break;
      case 'team2':
        this._setBracketWinner(2);
        break;
    }
  }

  public fetchData(): Promise<void>{
    return new Promise((res, rej) => {
      if(this._hasFetchedData)return res();
      this._hasFetchedData = true;

      window.CacheManager.get(window.ENDPOINT + '/api/v1/matches/selected')
        .then(data => {
          if(!data.ok){
            alert(data.error);
            rej(data.error);

            return;
          }

          this._setIsPlaying(data.playing);

          if(data.playing){
            window.CacheManager.get(window.ENDPOINT + '/api/v1/brackets/current')
              .then(data => {
                this.setTeam1And2(data.current[0], data.current[1]);
                this.setNextTeam1And2(data.next[0], data.next[1]);

                switch(data.current[2]){
                  case 1:
                    this.setWinningTeam('team1');
                    break;
                  case 2:
                    this.setWinningTeam('team2');
                    break;
                }
              });
          }

          if(data.match){
            this._selectedMatchData = data.match;
            this._matchStatusText.innerText = `Selected Match: ${this._selectedMatchData!.name}`;

            if(this._matchChangeCB)this._matchChangeCB(this._selectedMatchData);
            if(this._bracketsHookCB)this._bracketsHookCB(this._selectedMatchData);
          }

          window.CacheManager.get(window.ENDPOINT + '/api/v1/matches/list')
            .then(async data => {
              if(!data.ok){
                console.error(data);
                rej(data.error);

                return;
              }

              this._matches = data.matches;

              data.matches.forEach(( match: Match ) => {
                this._matchList[match._id] = <div
                  class={ "match" + ( this._selectedMatchData && this._selectedMatchData._id === match._id ? ' match-selected' : '' ) }
                  onClick={() => window.MatchManager.selectMatch(match._id)}
                >
                  { match.name }
                </div> as HTMLDivElement;
              })

              if(this._matchHTMLList){
                this._matchHTMLList.innerHTML = '';
                Object.values(this._matchList).forEach(( el: any ) => { this._matchHTMLList!.appendChild(el); })
              }

              this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));
              this.loaded = true;

              res();
            })
            .catch(console.error);
        })
        .catch(console.error);
    })
  }

  public selectMatchDisplay( id: string ){
    if(this._selectedMatchData)
      this._matchList[this._selectedMatchData._id].classList.remove('match-selected');

    this._selectedMatchData = this._matches.find(x => x._id === id) || null;

    if(this._selectedMatchData){
      this._matchStatusText.innerText = `Selected Match: ${this._selectedMatchData.name}`
      this._matchList[id].classList.add('match-selected');
    } else{
      this._matchStatusText.innerText = `No Match Selected.`
    }

    if(this._matchChangeCB)this._matchChangeCB(this._selectedMatchData);
    if(this._bracketsHookCB)this._bracketsHookCB(this._selectedMatchData);

    this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));
  }

  public selectMatch( id: string ){
    if(this._selectedMatchData)
      this._matchList[this._selectedMatchData._id].classList.remove('match-selected');

    if(
      this._selectedMatchData &&
      this._selectedMatchData._id === id
    )
      this._selectedMatchData = null;
    else
      this._selectedMatchData = this._matches.find(x => x._id === id) || null;

    if(this._selectedMatchData){
      this._matchStatusText.innerText = `Selected Match: ${this._selectedMatchData.name}`
      this._matchList[id].classList.add('match-selected');
    } else{
      this._matchStatusText.innerText = `No Match Selected.`
    }

    if(this._matchChangeCB)this._matchChangeCB(this._selectedMatchData);
    if(this._bracketsHookCB)this._bracketsHookCB(this._selectedMatchData);

    fetch(window.ENDPOINT + '/api/v1/matches/select', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cooki.getStore('token')}`
      },
      body: JSON.stringify({ id: this._selectedMatchData ? this._selectedMatchData._id : '' })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok)
          return alert(data.error);
      })
  }

  public addMatch( match: Match ){
    if(this._matches.find(x => x._id === match._id))return;

    this._matches.push(match);
    this._matchList[match._id] = <div
      class="match"
      onClick={() => window.MatchManager.selectMatch(match._id)}
    >
      { match.name }
    </div> as HTMLDivElement;

    if(this._matchHTMLList)
      this._matchHTMLList.appendChild(this._matchList[match._id]);

    this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));
  }

  public matchListContainer(): HTMLDivElement[] {
    return Object.values(this._matchList);
  }

  public containerREF( el: HTMLDivElement ){
    this._matchHTMLList = el;

    if(this._matchHTMLList){
      this._matchHTMLList.innerHTML = '';
      Object.values(this._matchList).forEach(( el: any ) => { this._matchHTMLList!.appendChild(el); })
    }
  }

  public onMatchChange( cb: ( match: Match | null ) => void ){
    this._matchChangeCB = cb;
    this._matchChangeCB(this._selectedMatchData);
  }

  public onMatchesChange( cb: ( match: Match[], selected: Match | null ) => void ){
    this._matchesChangeCB.push(cb);
    cb(this._matches, this._selectedMatchData);
  }

  public onBracketsHook( cb: ( selected: Match | null ) => void ){
    this._bracketsHookCB = cb;
    cb(this._selectedMatchData);
  }

  public renameDisplay( id: string, name: string ){
    this._matches.find(x => x._id === id)!.name = name;
    this._matchList[id].innerHTML = name;

    this._matchStatusText.innerText = `Selected Match: ${name}`;
    this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));

    if(this._matchChangeCB)this._matchChangeCB(this._selectedMatchData);
    if(this._bracketsHookCB)this._bracketsHookCB(this._selectedMatchData);

    this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));
  }

  public renameSelected( name: string ){
    if(!this._selectedMatchData)return;

    this._selectedMatchData.name = name;
    this._matchList[this._selectedMatchData._id].innerHTML = name;

    this._matchStatusText.innerText = `Selected Match: ${name}`;

    fetch(window.ENDPOINT + '/api/v1/matches/rename', {
      method: 'PUT',
      headers: {
        'Authorization': `Bearer ${cooki.getStore('token')}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ name, id: this._selectedMatchData._id })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }
      })
  }

  public selected(): Match | null{
    return this._selectedMatchData;
  }

  public deleteDisplay( id: string ){
    let matchData = this._matches.find(x => x._id === id);
    if(!matchData)return;

    this._matches = this._matches.filter(x => x._id !== matchData._id);

    this._matchList[id].remove();
    delete this._matchList[id];

    if(matchData._id === this._selectedMatchData?._id){
      this._selectedMatchData = null;
      this._matchStatusText.innerText = `No Match Selected.`

      if(this._matchChangeCB)this._matchChangeCB(null);
      if(this._bracketsHookCB)this._bracketsHookCB(null);
    }

    this._matchesChangeCB.forEach(cb => cb(this._matches, this._selectedMatchData));
  }

  public deleteSelected(){
    if(!this._selectedMatchData)return;
    let matchData = this._selectedMatchData;

    this._matches = this._matches.filter(x => x._id !== matchData._id);

    this._matchList[this._selectedMatchData._id].remove();
    delete this._matchList[this._selectedMatchData._id];

    this._selectedMatchData = null;
    this._matchStatusText.innerText = `No Match Selected.`

    if(this._matchChangeCB)
      this._matchChangeCB(null);

    fetch(window.ENDPOINT + '/api/v1/matches/delete?id=' + matchData._id, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${cooki.getStore('token')}`,
        'Content-Type': 'application/json'
      }
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }
      })
  }

  public get( id: string ): Match | null{
    return this._matches.find(x => x._id === id) || null;
  }

  public static Init(
    matchStatusText: HTMLDivElement
  ){
    window.MatchManager = new MatchManager(matchStatusText); // When "Init" is called create a new instance of MatchManager and fill in "window.MatchManager"
  }
}

export { MatchManager } // Export MatchManager so other scripts can use the type / call static functions