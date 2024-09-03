import { onMount } from "solid-js";
import './Dash.css'
import * as cooki from '../managers/CookiManager';
import { useNavigate } from "@solidjs/router";
import { SideBarButton } from "../managers/SideBarManager";

let Dash = () => {
  let natigate = useNavigate();

  onMount(() => {
    let token = cooki.getStore('token');
    if(!token)return natigate('/');

    window.CacheManager.get(window.ENDPOINT + '/api/v1/auth/verify')
      .then(data => {
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

        window.SideBarManager.open();
        window.MatchManager.fetchData();
      })
      .catch(console.error);
  })

  return (
    <>

    </>
  )
}

export default Dash;