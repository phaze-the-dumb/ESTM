import { Bracket } from "../structs/Bracket";
import { Match } from "../structs/Match";
import { Team } from "../structs/Team";
import * as cooki from './CookiManager';

// Add an element to the global "window" variable to store the object
declare global{
  interface Window{
    BracketDiagramManager: BracketDiagramManager
  }
}

class BracketDiagramManager{
  private _canvas!: HTMLCanvasElement;
  private _ctx!: CanvasRenderingContext2D;

  private _offsetX: number = 0;
  private _offsetY: number = 0;

  private _isMouseDown: boolean = false;
  private _lastMousePos: number[] = [ 0, 0 ];

  private _brackets: Bracket[][] = [];
  private _teams: Team[] = [];

  private _stopRender = true;
  // private _frames = 0;

  constructor(){
    window.MatchManager.onBracketsHook(( selected ) => this.fetchData(selected));

    // setInterval(() => {
    //   console.log(this._frames);
    //   this._frames = 0;
    // }, 1000);

    window.addEventListener('resize', () => {
      if(!this._canvas)return;
      let rect = this._canvas.getBoundingClientRect();

      this._canvas.width = rect.width;
      this._canvas.height = rect.height;
    })

    window.onmouseup = () => this._isMouseDown = false;

    window.onmousemove = ( e ) => {
      if(this._isMouseDown){
        this._offsetX += this._lastMousePos[0] - e.clientX;
        this._offsetY += this._lastMousePos[1] - e.clientY;

        this._lastMousePos = [ e.clientX, e.clientY ];
      }
    }
  }

  resize(){
    let rect = this._canvas.getBoundingClientRect();

    this._canvas.width = rect.width;
    this._canvas.height = rect.height;
  }

  fetchData( selected: Match | null ){
    if(!selected){
      this._teams = [];
      this._brackets = [];

      return;
    }
    this.stop();

    let loadedAmount = 0;

    fetch(window.ENDPOINT + '/api/v1/teams/list?match_id=' + selected._id, {
      headers: {
        'Authorization': `Bearer ${cooki.getStore('token')}`,
      }
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok)
          return alert(data.error);

        this._teams = data.teams;

        loadedAmount++;
        if(loadedAmount === 2 && this._canvas)
          this.start(this._canvas);
      })

    fetch(window.ENDPOINT + '/api/v1/brackets/get_match?match_id=' + selected._id, {
      headers: {
        'Authorization': `Bearer ${cooki.getStore('token')}`,
      }
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok)
          return alert(data.error);

        this._brackets = [];

        let set: Bracket[] = [];
        let setIndex = '0';

        data.brackets.forEach(( bracket: any ) => {
          let splitId = bracket._id.split(':');

          if(setIndex !== splitId[0]){
            this._brackets.push(set);
            setIndex = splitId[0];

            set = [];
          }

          set.push(bracket);
        })

        this._brackets.push(set);

        loadedAmount++;
        if(loadedAmount === 2 && this._canvas)
          this.start(this._canvas);
      })
  }

  render(){
    if(this._stopRender)return;

    // this._frames++;
    this._ctx.clearRect(0, 0, this._canvas.width, this._canvas.height);

    this._ctx.font = '20px Rubik';

    let yPos: number[][] = [];

    for (let set = 1; set < this._brackets.length; set++) {
      let prevSet = this._brackets[set - 1];
      let positions = [];

      for (let i = 0; i < this._brackets[set].length; i++) {
        let bracket = this._brackets[set][i];

        this._ctx.shadowBlur = 10;
        this._ctx.shadowColor = '#0005';

        let yDrop;

        if(set === 1){
          yDrop = i * 100;
          positions.push(yDrop);
        } else{
          let topPos = yPos[set - 2][bracket.team1];

          let bottomPos
          if(bracket.team2 !== -1)
            bottomPos = yPos[set - 2][bracket.team2];
          else
            bottomPos = topPos;

          yDrop = (topPos + ( bottomPos - topPos ) / 2);

          if(set !== 1){
            this._ctx.strokeStyle = '#1B263B';
            this._ctx.lineWidth = 5;

            this._ctx.beginPath();
            this._ctx.moveTo((100 + (set - 1) * 300) - this._offsetX, (137.5 + yDrop) - this._offsetY);
            this._ctx.lineTo((75 + (set - 1) * 300) - this._offsetX, (137.5 + yDrop) - this._offsetY);
            this._ctx.stroke();
            this._ctx.closePath();

            this._ctx.beginPath();
            this._ctx.moveTo((75 + (set - 1) * 300) - this._offsetX, 137.5 + topPos - this._offsetY);
            this._ctx.lineTo((75 + (set - 1) * 300) - this._offsetX, 137.5 + bottomPos - this._offsetY);
            this._ctx.stroke();
            this._ctx.closePath();
  
            this._ctx.beginPath();
            this._ctx.moveTo((75 + (set - 1) * 300) - this._offsetX, 137.5 + topPos - this._offsetY);
            this._ctx.lineTo((50 + (set - 1) * 300) - this._offsetX, 137.5 + topPos - this._offsetY);
            this._ctx.stroke();
            this._ctx.closePath();

            this._ctx.beginPath();
            this._ctx.moveTo((75 + (set - 1) * 300) - this._offsetX, 137.5 + bottomPos - this._offsetY);
            this._ctx.lineTo((50 + (set - 1) * 300) - this._offsetX, 137.5 + bottomPos - this._offsetY);
            this._ctx.stroke();
            this._ctx.closePath();
          }

          positions.push(yDrop);
        }

        this._ctx.fillStyle = '#1B263B';
        this._ctx.fillRect((100 + (set - 1) * 300) - this._offsetX, (100 + yDrop) - this._offsetY, 250, 70);

        this._ctx.shadowBlur = 0;
        this._ctx.fillStyle = '#E0E1DD';

        if(prevSet[bracket.team1].winner !== -1)
          this._ctx.fillText(this._teams[prevSet[bracket.team1].winner].name, (110 + (set - 1) * 300) - this._offsetX, (125 + yDrop) - this._offsetY);
        else{
          this._ctx.fillStyle = '#778DA9';
          this._ctx.fillText('Winner of bracket #' + (bracket.team1 + 1), (110 + (set - 1) * 300) - this._offsetX, (125 + yDrop) - this._offsetY);
        }

        this._ctx.fillStyle = '#E0E1DD';

        if(bracket.team2 !== -1){
          if(prevSet[bracket.team2].winner !== -1)
            this._ctx.fillText(this._teams[prevSet[bracket.team2].winner].name, (110 + (set - 1) * 300) - this._offsetX, (160 + yDrop) - this._offsetY);
          else{
            this._ctx.fillStyle = '#778DA9';
            this._ctx.fillText('Winner of bracket #' + (bracket.team2 + 1), (110 + (set - 1) * 300) - this._offsetX, (160 + yDrop) - this._offsetY);
          }
        }
      }

      yPos.push(positions);
      positions = [];
    }

    requestAnimationFrame(() => this.render());
  }

  public start( canvas: HTMLCanvasElement ){
    this._canvas = canvas;
    this._ctx = this._canvas.getContext('2d')!;

    this._stopRender = false;

    let rect = this._canvas.getBoundingClientRect();

    this._canvas.width = rect.width;
    this._canvas.height = rect.height;

    canvas.onmousedown = ( e ) => {
      this._isMouseDown = true;
      this._lastMousePos = [ e.clientX, e.clientY ];
    }

    requestAnimationFrame(() => this.render());
  }

  public stop(){
    this._stopRender = true;
  }

  public static Init(){
    if(window.BracketDiagramManager)return;
    window.BracketDiagramManager = new BracketDiagramManager();
  }
}

export { BracketDiagramManager }