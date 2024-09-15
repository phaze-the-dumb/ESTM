import {createSignal, Match, onMount, Show, Switch} from "solid-js";
import './Dash.css'
import * as cooki from '../managers/CookiManager';
import { useNavigate } from "@solidjs/router";
import { SideBarButton } from "../managers/SideBarManager";

let Dash = () => {
  let natigate = useNavigate();
  let [ teamCount, setTeamCount ] = createSignal(0);

  onMount(() => {
    let token = cooki.getStore('token');
    if(!token)return natigate('/');

    window.CacheManager.get(window.ENDPOINT + '/api/v1/auth/verify')
      .then(async data => {
        if(!data.ok){
          cooki.tryRemoveStore('token');
          natigate('/');

          return;
        }

        window.SideBarManager.setButtons([
          new SideBarButton("Overview", () => {
            natigate('/dash');
          }),
          new SideBarButton("Matches", () => {
            natigate('/dash/matches');
          }),
          new SideBarButton("Teams", () => {
            natigate('/dash/teams');
          }),
          new SideBarButton("Brackets", () => {
            natigate('/dash/brackets');
          }),
        ], "dash");


        // if(cooki.getStore('token'))
        //   window.LiveDataManager.sendHello();

        window.SideBarManager.open();
        await window.MatchManager.fetchData();

        let match = window.MatchManager.selected();
        if(match){
          window.CacheManager.get(window.ENDPOINT + '/api/v1/teams/list?match_id=' + match._id)
            .then(data => {
              setTeamCount(data.teams.length);
            })
        }
      })
      .catch(console.error);
  })

  return (
    <>
      <div class="overview-header">
        <h1>Overview</h1>

        <div class="overview-row">
          <div class="overview-column">
            Team Count:<br />

          </div>
          <div class="overview-column">
            {teamCount()}<br />

          </div>
        </div>
      </div>

      <Show when={!window.MatchManager.isPlaying()}>
        <div style={{ "text-align": 'center', "margin-top": '25px' }}>
          <div class="button" onClick={() => window.ConfirmationManager.show(<div>Are you sure you want to start the match?<br />You will <b>NOT</b> be able to edit any teams / matches until the game is over.</div> as HTMLElement, () => {
            fetch(window.ENDPOINT + '/api/v1/matches/start', {
              method: 'POST',
              headers: {
                'Authorization': `Bearer ${cooki.getStore('token')}`
              }
            })
              .then(data => data.json())
              .then(data => {
                if(!data.ok){
                  alert(data.error);
                  return;
                }
              })
          })}>Start Match</div>
        </div>
      </Show>

      <Show when={window.MatchManager.isPlaying()}>
        <div style={{ "text-align": 'center', "margin-top": '25px' }}>
          <div class="button-danger" onClick={() => window.ConfirmationManager.show(<div>Are you sure you want to cancel the match?</div> as HTMLElement, () => {
            fetch(window.ENDPOINT + '/api/v1/matches/cancel', {
              method: 'POST',
              headers: {
                'Authorization': `Bearer ${cooki.getStore('token')}`
              }
            })
              .then(data => data.json())
              .then(data => {
                if(!data.ok){
                  alert(data.error);
                  return;
                }
              })
          })}>Cancel Match</div>
        </div>
      </Show><br /><br />

      <div class="match-stats">
        <Show when={window.MatchManager.isPlaying()} fallback={
          <div class="match-waiting">
            <div>
              <img src="/pause-solid.svg" width="300px" />
              <div>Waiting for a match to be started...</div>
            </div>
          </div>
        }>
          <div class="match-stats-container">
            <div class="match-stats-current">
              <div class="match-stats-waiting-smol">Currently Playing</div>

              <Switch>
                <Match when={window.MatchManager.bracketWinner() === 0}>
                  <Show when={
                    window.MatchManager.playingTeam1() &&
                    window.MatchManager.playingTeam2()
                  } fallback={
                    <div class="match-stats-waiting">Waiting for Teams...</div>
                  }>
                    <div class="column">
                      <div class="row">
                        <div class="match-stats-waiting">{ window.MatchManager.playingTeam1()!.name }</div>

                        <div class="button" onClick={() => {
                          fetch(window.ENDPOINT + '/api/v1/brackets/winner', {
                            method: 'PUT',
                            headers: {
                              'Authorization': `Bearer ${cooki.getStore('token')}`,
                              'Content-Type': 'application/json'
                            },
                            body: JSON.stringify({ team: 'team1' })
                          })
                            .then(data => data.json())
                            .then(data => {
                              if(!data.ok){
                                alert(data.error);
                                return;
                              }
                            })
                        }}>{ window.MatchManager.playingTeam1()!.name } Win!</div>
                      </div>
                      <div class="row">
                        <div class="match-stats-waiting">{ window.MatchManager.playingTeam2()!.name }</div>

                        <div class="button" onClick={() => {
                        fetch(window.ENDPOINT + '/api/v1/brackets/winner', {
                          method: 'PUT',
                          headers: {
                            'Authorization': `Bearer ${cooki.getStore('token')}`,
                            'Content-Type': 'application/json'
                          },
                          body: JSON.stringify({ team: 'team2' })
                        })
                          .then(data => data.json())
                          .then(data => {
                            if(!data.ok){
                              alert(data.error);
                              return;
                            }
                          })
                        }}>{ window.MatchManager.playingTeam2()!.name } Win!</div>
                      </div>
                    </div>
                  </Show>
                </Match>
                <Match when={window.MatchManager.bracketWinner() === 1}>
                  <div class="match-stats-team-won match-stats-waiting">
                    { window.MatchManager.playingTeam1()!.name } Won!
                  </div>

                  <Show when={
                    !window.MatchManager.playingNextTeam1() ||
                    !window.MatchManager.playingNextTeam2()
                  }>
                    <div class="button" onClick={() => {
                      fetch(window.ENDPOINT + '/api/v1/matches/cancel', {
                        method: 'POST',
                        headers: {
                          'Authorization': `Bearer ${cooki.getStore('token')}`
                        }
                      })
                        .then(data => data.json())
                        .then(data => {
                          if(!data.ok){
                            alert(data.error);
                            return;
                          }
                        })
                    }}>Finish Match.</div>
                  </Show>
                </Match>
                <Match when={window.MatchManager.bracketWinner() === 2}>
                  <div class="match-stats-team-won match-stats-waiting">
                    { window.MatchManager.playingTeam2()!.name } Won!
                  </div>

                  <Show when={
                    !window.MatchManager.playingNextTeam1() ||
                    !window.MatchManager.playingNextTeam2()
                  }>
                    <div class="button" onClick={() => {
                      fetch(window.ENDPOINT + '/api/v1/matches/cancel', {
                        method: 'POST',
                        headers: {
                          'Authorization': `Bearer ${cooki.getStore('token')}`
                        }
                      })
                        .then(data => data.json())
                        .then(data => {
                          if(!data.ok){
                            alert(data.error);
                            return;
                          }
                        })
                    }}>Finish Match.</div>
                  </Show>
                </Match>
              </Switch>
            </div>
            <div class="match-stats-next">
              <div class="match-stats-waiting-smol">Next Up</div>

              <Show when={
                window.MatchManager.playingNextTeam1() &&
                window.MatchManager.playingNextTeam2()
              } fallback={
                <div class="match-stats-waiting">Waiting for Teams...</div>
              }>
                <div>
                  <div class="column">
                    <div class="row">
                      <div class="match-stats-waiting-smol">Team 1</div>
                      <div class="match-stats-waiting">{ window.MatchManager.playingNextTeam1()!.name }</div>
                    </div>
                    <div class="row">
                      <div class="match-stats-waiting-smol">Team 2</div>
                      <div class="match-stats-waiting">{ window.MatchManager.playingNextTeam2()!.name }</div>
                    </div>
                  </div>

                  <div class="button" onClick={() => {
                    fetch(window.ENDPOINT + '/api/v1/matches/next', {
                      method: 'POST',
                      headers: {
                        'Authorization': `Bearer ${cooki.getStore('token')}`
                      }
                    })
                      .then(data => data.json())
                      .then(data => {
                        if(!data.ok){
                          alert(data.error);
                          return;
                        }
                      })
                  }}>Ready!</div>
                </div>
              </Show>
            </div>
          </div>
        </Show>
      </div>
    </>
  )
}

export default Dash;