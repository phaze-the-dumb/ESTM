import { onMount } from 'solid-js';
import anime from 'animejs';
import * as cooki from '../managers/CookiManager';
import { useNavigate } from '@solidjs/router';
import './Login.css';

let Login = () => {
  let navigate = useNavigate();

  let inputs: HTMLInputElement[] = [];

  let mainContainer: HTMLDivElement;
  let errorContainer: HTMLDivElement;

  let token = cooki.getStore('token');
  if(token){
    fetch(window.ENDPOINT + '/api/v1/auth/verify', {
      headers: {
        Authorization: 'Bearer ' + token
      }
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          cooki.tryRemoveStore('token');
          cooki.tryRemoveStore('token-id');

          return;
        }

        navigate('/dash');
      })
      .catch(e => {
        console.error(e);
      })
  }

  let submit = () => {
    let code = inputs.map(x => x.value).join('');

    fetch(window.ENDPOINT + '/api/v1/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      },
      body: JSON.stringify({ code })
    })
      .then(data => data.json())
      .then(data => {
        if(!data.ok){
          anime.set(errorContainer!, { opacity: 0, translateX: '-50%', translateY: '100px', rotate: '10deg' });

          errorContainer!.style.display = 'flex';
          errorContainer!.innerHTML = data.error;

          anime({
            targets: errorContainer!,
            opacity: 1,
            translateY: '145px',
            rotate: '0deg'
          });

          return;
        }

        cooki.setStore('token', data.token);
        cooki.setStore('token-id', data.id);

        window.MatchManager.fetchData();
        window.LiveDataManager.sendHello();

        navigate('/dash');
      })
      .catch(e => {
        anime.set(errorContainer!, { opacity: 0, translateX: '-50%', translateY: '100px', rotate: '10deg' });

        errorContainer!.style.display = 'flex';
        errorContainer!.innerHTML = e.toString();

        anime({
          targets: errorContainer!,
          opacity: 1,
          translateY: '145px',
          rotate: '0deg'
        });

        return;
      })
  }

  onMount(() => {
    window.SideBarManager.close();

    let lastKeyevent: KeyboardEvent;
    for (let i = 0; i < inputs.length; i++) {
      let input = inputs[i];

      input.onkeydown = ( e ) => {
        lastKeyevent = e;

        if(lastKeyevent.key === "Backspace" && input.value === ''){
          if(!inputs[i - 1])return;
          inputs[i - 1].select();
        }
      }

      input.oninput = () => {
        if(input.value.length > 1){
          let val = input.value;
          for (let j = 0; j < val.length; j++) {
            if(inputs[j + i])
              inputs[j + i].value = val[j];

            if(inputs[j + i + 1])
              inputs[j + i + 1].select();
            else
              submit();
          }
        } else{
          if(inputs[i + 1] && lastKeyevent.key.length === 1 && lastKeyevent.key !== "Backspace")
            inputs[i + 1].select();
          else if(i === 5 && input.value !== '')
            submit();
        }
      }
    }
  })

  return (
    <>
      <div class="login-page" ref={mainContainer!}>
        <h1>Login</h1>
        <p>Enter the code displayed in the app to authenticate this browser</p>

        <div style={{ "margin-top": '10px' }}>
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
          <input class="code-number-input" placeholder="-" ref={( el ) => inputs.push(el)} />
        </div>

        <div class="button" onClick={submit}>Login</div>
      </div>
      <div class="login-error" ref={errorContainer!}></div>
    </>
  )
}

export default Login