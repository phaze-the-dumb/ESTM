import {createSignal, onMount} from "solid-js";
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

        if(cooki.getStore('token'))
          window.LiveDataManager.sendHello();

        window.SideBarManager.open();
        await window.MatchManager.fetchData();

        let match = window.MatchManager.selected();
        if(match){
          window.CacheManager.get(window.ENDPOINT + '/api/v1/teams/list?match_id=' + match._id)
            .then(data => {
              console.log(data);
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
    </>
  )
}

export default Dash;