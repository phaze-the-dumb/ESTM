// Add an element to the global "window" variable so all components can access the sidebar manager class
declare global{
  interface Window{
    SideBarManager: SideBarManager;
  }
}

class SideBarButton{
  el: HTMLDivElement;

  constructor( label: string, cb: () => void ){
    this.el = <div onClick={cb} class="sidebar-button">{ label }</div> as HTMLDivElement;
  }
}

class SideBarManager{
  private closeCallback: () => void;
  private openCallback: () => void;
  private contentChangeCallback: ( buttons: SideBarButton[] ) => void;

  private _open: boolean = false;
  private _buttonSetId: string = "";

  public open(){
    if(this._open === true)return;
    this._open = true;

    this.openCallback();
  }

  public close(){
    if(this._open === false)return;
    this._open = false;

    this.closeCallback();
  }

  public setButtons( buttons: SideBarButton[], buttonSetId: string ){
    if(buttonSetId !== this._buttonSetId){
      this._buttonSetId = buttonSetId;
      this.contentChangeCallback(buttons);
    }
  }

  private constructor(
    closeCallback: () => void,
    openCallback: () => void,
    contentChangeCallback: ( buttons: SideBarButton[] ) => void
  ){
    this.closeCallback = closeCallback;
    this.openCallback = openCallback;
    this.contentChangeCallback = contentChangeCallback;
  }

  public static Init(
    closeCallback: () => void,
    openCallback: () => void,
    contentChangeCallback: ( buttons: SideBarButton[] ) => void
  ){
    window.SideBarManager = new SideBarManager(
      closeCallback,
      openCallback,
      contentChangeCallback
    ); // When "Init" is called create a new instance of SideBarManager and fill in "window.SideBarManager"
  }
}

export { SideBarManager, SideBarButton } // Export SideBarManager so other scripts can use the type / call static functions