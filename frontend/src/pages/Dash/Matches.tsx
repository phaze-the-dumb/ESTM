import { onMount } from 'solid-js';
import { useNavigate } from "@solidjs/router";
import { SideBarButton } from '../../managers/SideBarManager';
import * as cooki from '../../managers/CookiManager';
import { Match } from '../../structs/Match';
import anime from 'animejs';

import './Matches.css';

let Matches = () => {
  let natigate = useNavigate();

  let matchCreateContainer: HTMLDivElement;
  let matchCreateNameInput: HTMLInputElement;
  let matchCreateNameInputSubmit: HTMLDivElement;
  let matchCreateNameInputSubmitted = false;

  let matchCreateBackButton: HTMLDivElement;

  let matchContainer: HTMLDivElement;

  let matchTitle: HTMLDivElement;
  let matchTitleEdit: HTMLInputElement;

  let matchCreateNameSubmit = () => {
    let name = matchCreateNameInput.value;
    if(matchCreateNameInputSubmitted || name.trim().length === 0)return;

    matchCreateNameInputSubmitted = true;
    matchCreateNameInput.disabled = true;

    matchCreateNameInputSubmit.innerText = 'Loading...';

    console.log(name);

    fetch(window.ENDPOINT + '/api/v1/matches/create', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cooki.getStore('token')}`
      },
      body: JSON.stringify({ name })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          alert(data.error);
          return;
        }

        window.MatchManager.addMatch(new Match(data._id, name));

        anime({
          targets: matchCreateContainer,
          opacity: 0,
          easing: 'easeInOutQuad',
          duration: 100,
          complete: () => {
            matchCreateNameInput.value = '';
    
            matchCreateContainer.style.display = 'none';
            matchCreateNameInputSubmit.innerText = 'Create Match';
    
            matchCreateNameInputSubmitted = false;
            matchCreateNameInput.disabled = false;
          }
        });
      })
      .catch(e => {
        console.error(e);
        alert('Failed to create match: ' + e);
      })
  }

  let addMatch = () => {
    matchCreateContainer.style.display = 'block';

    anime({
      targets: matchCreateContainer,
      opacity: 1,
      easing: 'easeInOutQuad',
      duration: 100
    });
  }

  onMount(() => {
    matchCreateNameInput.onchange = matchCreateNameSubmit;
    matchCreateNameInputSubmit.onclick = matchCreateNameSubmit;

    matchCreateBackButton.onclick = () => {
      anime({
        targets: matchCreateContainer,
        opacity: 0,
        easing: 'easeInOutQuad',
        duration: 100,
        complete: () => {
          matchCreateNameInput.value = '';
          matchCreateContainer.style.display = 'none';
        }
      });
    }

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

    let selectedMatch: Match | null = null;
    matchTitleEdit.style.display = 'none';

    matchTitle.onclick = () => {
      if(!selectedMatch)return;

      matchTitleEdit.style.display = 'inline-block';
      matchTitle.style.display = 'none';

      matchTitleEdit.select();
    }

    matchTitleEdit.onchange = () => {
      matchTitleEdit.style.display = 'none';
      matchTitle.style.display = 'inline-block';

      window.MatchManager.renameSelected(matchTitleEdit.value);
      matchTitle.innerHTML = matchTitleEdit.value;
    }

    matchTitleEdit.onkeyup = ( e ) => {
      if(e.key === 'Enter'){
        matchTitleEdit.style.display = 'none';
        matchTitle.style.display = 'inline-block';

        window.MatchManager.renameSelected(matchTitleEdit.value);
        matchTitle.innerHTML = matchTitleEdit.value;
      }
    }

    window.MatchManager.onMatchChange(( match ) => {
      selectedMatch = match;

      if(match === null){
        matchTitle.innerHTML = 'No Match Selected.';
        matchTitleEdit.value = 'Error.';

        matchTitleEdit.style.display = 'none';
        matchTitle.style.display = 'inline-block';

        matchContainer.style.display = 'none';

        return;
      }

      matchTitle.innerHTML = selectedMatch!.name;
      matchTitleEdit.value = selectedMatch!.name;

      matchContainer.style.display = 'block';

      matchTitleEdit.style.display = 'none';
      matchTitle.style.display = 'inline-block';
    })
  })

  return (
    <>
      <div class="matches-header"><h1>Matches</h1></div>

      <div class="match-container" ref={( el ) => window.MatchManager.containerREF(el)}></div>

      <div style={{ display: window.MatchManager.isPlaying() ? 'none' : 'block' }} class="match-create button" onClick={addMatch}>+ Add Match</div>

    <div style={{ display: window.MatchManager.isPlaying() ? 'none' : 'block' }} >
      <div class="match-control" ref={matchContainer!}>
          <div>
            <div class="match-title" ref={matchTitle!}>Match</div>
            <input class="match-title-edit" value="Match" ref={matchTitleEdit!} />
          </div>

          <br /><br />
          <div class="button-danger" onClick={() => {
            window.ConfirmationManager.show(
              <div>Are you sure you want to delete this match?<br />This will also delete any teams within this match.</div> as HTMLElement,
              () => window.MatchManager.deleteSelected()
            )
          }}>Delete Match</div>
        </div>
      </div>

      <div class="match-create-container" ref={matchCreateContainer!}>
        <div class="back-button" ref={matchCreateBackButton!}>&lt; Back</div>
        <div class="match-create-modal">
          <h2>Create Match</h2>
          <input ref={matchCreateNameInput!} type="text" style={{ "margin-top": '5px' }} placeholder="Enter Match Name / Label..." />

          <div ref={matchCreateNameInputSubmit!} class="button" style={{
            width: '100%',
            margin: '2px',
            "margin-top": '10px'
          }}>
            Create Match
          </div>
        </div>
      </div>
    </>
  )
}

export default Matches;