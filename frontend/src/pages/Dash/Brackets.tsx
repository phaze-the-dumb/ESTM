import { onCleanup, onMount } from 'solid-js';
import { useNavigate } from "@solidjs/router";
import { SideBarButton } from '../../managers/SideBarManager';
import * as cooki from '../../managers/CookiManager';

import './Brackets.css'
import { BracketDiagramManager } from '../../managers/BracketDiagramManager';

let Brackets = () => {
  let natigate = useNavigate();

  let canvas: HTMLCanvasElement;

  onMount(() => {
    let token = cooki.getStore('token');
    if(!token)return natigate('/');

    BracketDiagramManager.Init();

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

        setTimeout(() => {
          window.BracketDiagramManager.start(canvas!);
        }, 250)
      })
      .catch(console.error);
  })

  onCleanup(() => {
    window.BracketDiagramManager.stop();
  })

  return (
    <>
      <canvas ref={canvas!} class="bracket-canvas"></canvas>
    </>
  )
}

export default Brackets;