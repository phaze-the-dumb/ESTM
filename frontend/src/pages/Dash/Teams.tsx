import { onMount, Show } from 'solid-js';
import { useNavigate } from "@solidjs/router";
import { SideBarButton } from '../../managers/SideBarManager';
import * as cooki from '../../managers/CookiManager';
import anime from 'animejs';

import './Teams.css'
import { Match } from '../../structs/Match';
import { Team } from '../../structs/Team';
import { Player } from '../../structs/Player';

let Teams = () => {
  let natigate = useNavigate();

  let dropdownCurrent: HTMLSpanElement;
  let dropdownOptions: HTMLDivElement;

  let teamCreateContainer: HTMLDivElement;
  let teamCreateNameInput: HTMLInputElement;
  let teamCreateNameInputSubmit: HTMLDivElement;
  let teamCreateNameInputSubmitted = false;

  let teamCreateBackButton: HTMLDivElement;

  let teamsList: HTMLDivElement;
  let teamsListContainer: any = {};

  let teamTitle: HTMLDivElement;
  let teamTitleEdit: HTMLInputElement;

  let teamEditPlayers: HTMLDivElement;
  let teamEditContainer: HTMLDivElement;

  let teamColourInput: HTMLInputElement;

  let addTeamButton: HTMLDivElement;

  let dropdownOpen = false;
  let teams: Team[] = [];

  let currentEditingTeam: Team | null = null;
  let editingTempPlayer: Player | null = null;

  let selectedMatch: string | null = localStorage.getItem('selectedMatchTeamsList');

  window.LiveDataManager.teamSocketUpdate(( msg ) => {
    switch(msg.type){
      case 'team-colour':
        let team4 = teams.find(x => x._id === msg.team._id);
        if(!team4)return;

        team4.colour = msg.team.colour;
        break;
      case 'rename-team':
        let team = teams.find(x => x._id === msg.team._id);
        if(!team)return;

        team.name = msg.team.name;
        teamsListContainer[msg.team._id].firstChild.innerText = msg.team.name;

        break;
      case 'create-team':
        if(msg.team.match_id !== selectedMatch)return;
        console.log(msg);

        addTeamToUI(new Team(msg.team));
        break;
      case 'delete-team':
        teams = teams.filter(x => x._id !== msg.team._id);
        teamsListContainer[msg.team._id].remove();

        break;
      case 'delete-match':
        teams = teams.filter(x => x.match_id !== msg.match._id);

        break;
      case 'rename-player':
        let team2 = teams.find(x => x._id === msg.player._id);
        if(!team2)return;

        let player1 = team2.players.find(x => x._id === msg.player.player_id);
        if(player1)player1.name = msg.player.name;

        let el1 = document.querySelector(`#player-name-label-${msg.player._id}-${msg.player.player_id}`);
        if(el1)el1.innerHTML = msg.player.name;

        break;
      case 'remove-player':
        let team3 = teams.find(x => x._id === msg.player._id);
        if(!team3)return;

        team3.players = team3.players.filter(x => x._id !== msg.player.player_id);

        let el = document.querySelector(`#player-name-label-${msg.player._id}-${msg.player.player_id}`);
        if(el)el.remove();

        break;
      case 'add-player':
        let team1 = teams.find(x => x._id === msg.player._id);
        if(!team1)return;

        let p = new Player();

        p._id = msg.player.player_id;
        p.name = msg.player.name;

        team1.players.push(p)
        teamsListContainer[msg.player._id].children[1].appendChild(<div id={ `player-name-label-${msg.player._id}-${msg.player.player_id}` }>{ msg.player.name }</div> as Node);

        teamEditPlayers!.appendChild(<div>
          <input style={{ display: 'none' }} value={ msg.player.name } onKeyUp={( e ) => {
            if(e.key === "Enter"){
              e.currentTarget.style.display = 'none';
              e.currentTarget.parentElement!.lastElementChild!.setAttribute("style", "display: block;");
            }
          }} onChange={( e ) => {
            e.currentTarget.nextElementSibling!.innerHTML = e.target.value;
  
            e.currentTarget.style.display = 'none';
            e.currentTarget.parentElement!.lastElementChild!.setAttribute("style", "display: block;");
  
            fetch(window.ENDPOINT + '/api/v1/teams/player', {
              method: 'PUT',
              headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${cooki.getStore('token')}`
              },
              body: JSON.stringify({
                id: currentEditingTeam!._id,
                player_id: msg.player.player_id,
                name: e.target.value
              })
            })
              .then(data => data.json())
              .then(data => {
                if(!data.ok){
                  alert("Could not update player name: " + data.error);
                  return;
                }
              })
              .catch(e => {
                alert("Could not update player name: " + e);
              })
          }}></input>
          <div onClick={( e ) => {
            e.currentTarget.style.display = 'none';
            e.currentTarget.parentElement!.firstElementChild!.setAttribute("style", "display: inline-block;");
          }}>
            { msg.player.name }
          </div>
        </div> as Node);
        break;
    }
  })

  let addTeamToUI = ( team: Team ) => {
    teams.push(team);

    let name: HTMLDivElement;

    teamsList!.appendChild(<div class="team" ref={( el ) => teamsListContainer[team._id] = el}>
      <div class="team-name" ref={name!}>{ team.name }</div>
      { Team.formatPlayerList(team) }
      <div class="button" onClick={() => {
        team.name = name!.innerText;
        teamEditor(team)
      }}>Edit</div>
    </div> as HTMLElement);
  }

  let selectMatch = ( match: Match | null ) => {
    if(!window.MatchManager.loaded)return;
    console.log(match);

    if(match){
      dropdownCurrent!.innerText = match.name;
      addTeamButton!.style.display = 'block';

      localStorage.setItem('selectedMatchTeamsList', match._id);
      selectedMatch = match._id;
    } else{
      dropdownCurrent!.innerText = 'No Selected Match';
      addTeamButton!.style.display = 'none';

      localStorage.removeItem('selectedMatchTeamsList');
      selectedMatch = null;
    }

    fetchTeams();
  }

  let teamCreateNameSubmit = () => {
    let name = teamCreateNameInput!.value;
    if(teamCreateNameInputSubmitted || name.trim().length === 0)return;

    teamCreateNameInputSubmitted = true;
    teamCreateNameInput!.disabled = true;

    teamCreateNameInputSubmit!.innerText = 'Loading...';

    fetch(window.ENDPOINT + '/api/v1/teams/create', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cooki.getStore('token')}`
      },
      body: JSON.stringify({ name, match_id: selectedMatch })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }

        anime({
          targets: teamCreateContainer!,
          opacity: 0,
          easing: 'easeInOutQuad',
          duration: 100,
          complete: () => {
            teamCreateNameInput!.value = '';
    
            teamCreateContainer!.style.display = 'none';
            teamCreateNameInputSubmit!.innerText = 'Create Match';
    
            teamCreateNameInputSubmitted = false;
            teamCreateNameInput!.disabled = false;
          }
        });
      })
      .catch(e => {
        console.error(e);
        alert('Failed to create match: ' + e);
      })
  }

  let selectedTeamId: string | null = null;
  let teamEditor = ( team: Team ) => {
    teamEditContainer!.style.display = 'block';
    selectedTeamId = team._id;

    teamTitle!.innerHTML = team.name;
    teamTitleEdit!.value = team.name;

    teamColourInput!.value = team.colour;
    teamEditPlayers!.innerHTML = '';

    currentEditingTeam = team;

    team.players.forEach(p => {
      teamEditPlayers!.appendChild(<div>
        <input style={{ display: 'none' }} value={ p.name } onKeyUp={( e ) => {
          if(e.key === "Enter"){
            e.currentTarget.style.display = 'none';
            e.currentTarget.parentElement!.lastElementChild!.setAttribute("style", "display: block;");
          }
        }} onChange={( e ) => {
          e.currentTarget.nextElementSibling!.innerHTML = e.target.value;

          e.currentTarget.style.display = 'none';
          e.currentTarget.parentElement!.lastElementChild!.setAttribute("style", "display: block;");

          fetch(window.ENDPOINT + '/api/v1/teams/player', {
            method: 'PUT',
            headers: {
              'Content-Type': 'application/json',
              'Authorization': `Bearer ${cooki.getStore('token')}`
            },
            body: JSON.stringify({
              id: currentEditingTeam!._id,
              player_id: p._id,
              name: e.target.value
            })
          })
            .then(data => data.json())
            .then(data => {
              if(!data.ok){
                alert("Could not update player name: " + data.error);
              }
            })
            .catch(e => {
              alert("Could not update player name: " + e);
            })
        }}></input>
        <div onClick={( e ) => {
          e.currentTarget.style.display = 'none';
          e.currentTarget.parentElement!.firstElementChild!.setAttribute("style", "display: inline-block;");
        }}>
          { p.name }
        </div>
      </div> as Node);
    })

    anime({
      targets: teamEditContainer!,
      opacity: [ 0, 1 ],
      easing: 'easeInOutQuad',
      duration: 100
    })
  }

  let renameTeam = ( name: string ) => {
    if(!selectedTeamId)return;

    let dom = teamsListContainer[selectedTeamId];
    if(!dom)return;

    dom.firstChild.innerText = name;

    fetch(window.ENDPOINT + '/api/v1/teams/rename', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cooki.getStore('token')}`
      },
      body: JSON.stringify({ name, id: selectedTeamId })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }
      })
      .catch(e => {
        console.error(e);
      })
  }

  window.addEventListener('mousedown', ( e: any ) => {
    if(
      dropdownOpen &&
      e.target &&
      e.target.className !== "" &&
      e.target.className !== "teams-dropdown-option" &&
      e.target.className !== "teams-dropdown-current"
    ){
      dropdownOpen = false;

      anime({
        targets: dropdownOptions!,
        easing: 'easeInOutQuad',
        duration: 100,
        opacity: [ 1, 0 ],
        complete: () => {
          dropdownOptions!.style.display = 'none';
        }
      })
    }
  })

  let fetchTeams = async () => {
    if(!selectedMatch){
      teamsList!.innerHTML = 'No Match Selected.';
      return;
    }

    let req = await fetch(window.ENDPOINT + '/api/v1/teams/list?match_id=' + selectedMatch, { headers: { 'Authorization': `Bearer ${cooki.getStore('token')}` } });
    let res = await req.json();

    teamsList!.innerHTML = '';

    res.teams.forEach(( team: Team ) => {
      teams.push(team);

      let name: HTMLDivElement;

      teamsList!.appendChild(<div class="team" ref={( el ) => teamsListContainer[team._id] = el}>
        <div class="team-name" ref={name!}>{ team.name }</div>
        { Team.formatPlayerList(team) }
        <div class="button" onClick={() => {
          team.name = name!.innerText;
          teamEditor(team)
        }}>Edit</div>
      </div> as HTMLElement);
    })
  }

  onMount(() => {
    teamCreateNameInput!.onchange = teamCreateNameSubmit;
    teamCreateNameInputSubmit!.onclick = teamCreateNameSubmit;

    teamCreateBackButton!.onclick = () => {
      anime({
        targets: teamCreateContainer!,
        opacity: 0,
        easing: 'easeInOutQuad',
        duration: 100,
        complete: () => {
          teamCreateNameInput!.value = '';
          teamCreateContainer!.style.display = 'none';
        }
      });
    }

    window.MatchManager.onMatchesChange(( matches, selected ) => {
      if(matches.length === 0){
        dropdownOptions!.innerHTML = '';
        dropdownOptions!.appendChild(<div class="teams-dropdown-option">No Selected Match</div> as HTMLElement);

        dropdownCurrent!.innerHTML = 'No Selected Match';
        teamsList!.innerText = 'No Match Selected';

        selectMatch(null);
        return;
      }

      if(!selectedMatch)
        selectMatch(selected);
      else{
        let match = matches.find(x => x._id === selectedMatch);
        if(match){
          dropdownCurrent!.innerHTML = match.name;
          addTeamButton!.style.display = 'block';

          fetchTeams();
        } else
          selectMatch(selected);
      }

      dropdownOptions!.innerHTML = '';

      matches.forEach(match => {
        dropdownOptions!.appendChild(<div class="teams-dropdown-option" onClick={() => selectMatch(match)}>{ match.name }</div> as HTMLElement);
      })
    })

    anime.set(dropdownOptions!, { opacity: '0', display: 'none' });

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
      })
      .catch(console.error);

    teamTitleEdit!.style.display = 'none';

    teamTitle!.onclick = () => {
      if(!selectedMatch || window.MatchManager.isPlaying())return;

      teamTitleEdit!.style.display = 'inline-block';
      teamTitle!.style.display = 'none';

      teamTitleEdit!.select();
    }

    teamTitleEdit!.onchange = () => {
      teamTitleEdit!.style.display = 'none';
      teamTitle!.style.display = 'inline-block';

      teamTitle!.innerHTML = teamTitleEdit!.value;
      renameTeam(teamTitleEdit!.value);
    }

    teamTitleEdit!.onkeyup = ( e ) => {
      if(e.key === 'Enter'){
        teamTitleEdit!.style.display = 'none';
        teamTitle!.style.display = 'inline-block';

        teamTitle!.innerHTML = teamTitleEdit!.value;
        renameTeam(teamTitleEdit!.value);
      }
    }
  })

  let addTeam = () => {
    teamCreateContainer!.style.display = 'block';

    anime({
      targets: teamCreateContainer!,
      opacity: 1,
      easing: 'easeInOutQuad',
      duration: 100
    });
  }

  let submitTempPlayer = () => {
    document.querySelector("#temp-player-editor")!.remove();

    fetch(window.ENDPOINT + '/api/v1/teams/player', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cooki.getStore('token')}`
      },
      body: JSON.stringify({
        id: currentEditingTeam!._id,
        name: editingTempPlayer!.name
      })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }
      })
      .catch(e => {
        console.error(e);
      })
  }

  return (
    <>
      <div class="teams-header"><h1>Teams</h1></div>

      <div class="teams-search-bar">
        <input type="text" class="teams-search-bar-input" placeholder='Search Teams...' />
        <div class="teams-dropdown">
          <div class="teams-dropdown-current" onClick={() => {
            if(dropdownOpen){
              dropdownOpen = false;

              anime({
                targets: dropdownOptions!,
                easing: 'easeInOutQuad',
                duration: 100,
                opacity: [ 1, 0 ],
                complete: () => {
                  dropdownOptions!.style.display = 'none';
                }
              })
            } else{
              dropdownOpen = true;
              dropdownOptions!.style.display = 'block';

              anime({
                targets: dropdownOptions!,
                easing: 'easeInOutQuad',
                duration: 100,
                opacity: [ 0, 1 ]
              })
            }
          }}>
            <span ref={dropdownCurrent!}>Loading...</span>
            <div style={{ 'margin-left': '20px' }}></div>
            <img src="/caret-down-solid.svg" width="20" />
          </div>
          <div class="teams-dropdown-options" ref={dropdownOptions!}>
            <div class="teams-dropdown-option">No Selected Match</div>
          </div>
        </div>
      </div>

      <div class="teams-list" ref={teamsList!}>Loading...</div>

      <div style={{ display: window.MatchManager.isPlaying() ? 'none' : 'block' }}>
        <div class="team-create-button button" onClick={addTeam} ref={addTeamButton!} style={{ display: 'none' }}>+ Add Team</div>
      </div>

      <br /><br />

      <div class="team-create-container" ref={teamCreateContainer!}>
        <div class="back-button" ref={teamCreateBackButton!}>&lt; Back</div>
        <div class="team-create-modal">
          <h2>Create Team</h2>
          <input ref={teamCreateNameInput!} type="text" style={{ "margin-top": '5px' }} placeholder="Enter Team Name / Label..." />

          <div ref={teamCreateNameInputSubmit!} class="button" style={{
            width: '100%',
            margin: '2px',
            "margin-top": '10px'
          }}>
            Create Team
          </div>
        </div>
      </div>

      <div class="team-edit-container" ref={teamEditContainer!}>
        <div class="back-button" onClick={() => {
          anime({
            targets: teamEditContainer!,
            opacity: [ 1, 0 ],
            easing: 'easeInOutQuad',
            duration: 100,
            complete: () => {
              teamEditContainer!.style.display = 'none';
            }
          })
        }}>&lt; Back</div>

        <div class="team-edit-options">
          <div>
            <div>
              <div class="team-title" ref={teamTitle!}>Team</div>
              <input class="team-title-edit" ref={teamTitleEdit!} />
            </div><br /><br />

            <h3>Players <div class="add-team-player" onClick={() => {
              editingTempPlayer = new Player();

              teamEditPlayers!.appendChild(<div id="temp-player-editor">
                <input onChange={( el ) => {
                  editingTempPlayer!.name = el.target.value;
                  submitTempPlayer();
                }}></input>
              </div> as HTMLDivElement);
            }}>+</div></h3>
            <div ref={teamEditPlayers!}>
            </div>
            <br />
            <input type="color" ref={teamColourInput!} onChange={( el ) => {
              fetch(window.ENDPOINT + '/api/v1/teams/colour', {
                method: 'PUT',
                headers: {
                  Authorization: `Bearer ${cooki.getStore('token')}`,
                  'Content-Type': 'application/json',
                  'Accept': 'application/json'
                },
                body: JSON.stringify({
                  colour: el.currentTarget.value,
                  id: selectedTeamId
                })
              })
                .then(data => data.json())
                .then(data => {
                  if(!data.ok){
                    return alert("Error: " + data.error);
                  }
                })
                .catch(e => {
                  alert(e);
                })
            }}></input>

            <br /><br />

            <div style={{ display: window.MatchManager.isPlaying() ? 'none' : 'block' }}>
              <div class="button-danger" onClick={() => window.ConfirmationManager.show(
                <div>Are you sure you want to delete this team?</div> as HTMLElement,
                () => {
                  if(!selectedTeamId)return;

                  teamsListContainer[selectedTeamId].remove();
                  teams = teams.filter(x => x._id !== selectedTeamId);

                  anime({
                    targets: teamEditContainer!,
                    opacity: [ 1, 0 ],
                    easing: 'easeInOutQuad',
                    duration: 100,
                    complete: () => {
                      teamEditContainer!.style.display = 'none';
                    }
                  })

                  fetch(window.ENDPOINT + '/api/v1/teams/delete?id=' + selectedTeamId, {
                    method: 'DELETE',
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

                      selectedTeamId = null;
                    })
                    .catch(e => {
                      console.error(e);
                    })
                }
              )}>Delete</div>
            </div>
          </div>
        </div>

        <Show when={window.MatchManager.isPlaying()}>
          <div class="teams-not-editable-warning">
            You cannot edit this team as you are currently in play mode.
          </div>
        </Show>
      </div>
    </>
  )
}

export default Teams