import { SideBarManager } from './managers/SideBarManager';
import { onMount } from 'solid-js';
import anime from 'animejs';
import './App.css';
import { MatchManager } from './managers/MatchManager';
import { ConfirmationManager } from './managers/ConfirmationManager';
import { LiveDataManager } from './managers/LiveDataManager';

let App = ( props: any ) => {
  let topbar: HTMLDivElement;
  let sidebar: HTMLDivElement;
  let sidebarButtons: HTMLDivElement;
  let content: HTMLDivElement;

  onMount(() => {
    anime.set(sidebar, { left: '-250px' });
    anime.set(topbar, { top: '-50px' });
    anime.set(content, { left: '10px', width: 'calc(100vw - 20px)', top: '10px', height: 'calc(100% - 20px)' });

    MatchManager.Init(topbar);
    LiveDataManager.Init();

    SideBarManager.Init(
      () => {
        // Close Sidebar
        anime.set(content, { left: '260px', width: window.innerWidth - 270 + 'px' });

        anime({ targets: sidebar, left: '-250px', easing: 'easeInOutCirc', duration: 250 });
        anime({ targets: topbar, top: '-50px', easing: 'easeInOutCirc', duration: 250 });

        anime({
          targets: content,
          left: '10px',
          width: window.innerWidth - 20 + 'px',
          top: '10px',
          height: 'calc(100% - 20px)',
          easing: 'easeInOutCirc',
          duration: 250,
          complete: () => {
            content.style.width = 'calc(100vw - 20px)'
          }
        });
      },
      () => {
        // Open Sidebar
        anime.set(content, { left: '10px', width: window.innerWidth - 20 + 'px' });

        anime({ targets: sidebar, left: '0px', easing: 'easeInOutCirc', duration: 250 });
        anime({ targets: topbar, top: '0px', easing: 'easeInOutCirc', duration: 250 });

        anime({
          targets: content,
          left: '260px',
          width: window.innerWidth - 270 + 'px',
          top: '50px',
          height: 'calc(100% - 60px)',
          easing: 'easeInOutCirc',
          duration: 250,
          complete: () => {
            content.style.width = 'calc(100vw - 270px)'
          }
        });
      },
      ( buttons ) => {
        // Sidebar contents has changed

        sidebarButtons.innerHTML = '';
        buttons.forEach(btn => {
          sidebarButtons.appendChild(btn.el);
        })
      },
      () => {
        sidebar.style.display = 'none';
        topbar.style.display = 'none';

        content.style.left = '0';
        content.style.top = '0';

        content.style.width = '100%';
        content.style.height = '100%';

        content.style.background = 'rgba(0, 0, 0, 0)';

        content.classList.remove('content');
        document.body.style.background = '#000';
      }
    );
  })

  return (
    <>
      <div ref={ConfirmationManager.Init}></div>
      <div class="sidebar" ref={sidebar!}>
        <div class="sidebar-buttons" ref={sidebarButtons!}></div>
      </div>
      <div class="top-bar" ref={topbar!}>
        No Match Selected.
      </div>
      <div class="content" ref={content!}>
        { props.children }
      </div>
    </>
  )
}

export default App
