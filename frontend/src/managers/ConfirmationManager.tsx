import anime from "animejs";

// Add an element to the global "window" variable to store the cache
declare global{
  interface Window{
    ConfirmationManager: ConfirmationManager
  }
}

class ConfirmationManager{
  private _popup: HTMLDivElement;
  private _callback: () => void = () => {};
  private _textContainer!: HTMLDivElement;

  private constructor( container: HTMLDivElement ){
    this._popup = <div class="confirmation-popup" style={{ display: 'none' }}>
      <div class="confirmation-container">
        <div ref={this._textContainer}></div>

        <div class="button-danger" onClick={() => {
          anime({
            targets: this._popup,
            opacity: [ 1, 0 ],
            easing: 'easeInOutQuad',
            duration: 100,
            complete: () => {
              this._popup.style.display = 'none';
            }
          });

          this._callback()
        }}>Confirm</div>
        <div class="button" onClick={() => {
          anime({
            targets: this._popup,
            opacity: [ 1, 0 ],
            easing: 'easeInOutQuad',
            duration: 100,
            complete: () => {
              this._popup.style.display = 'none';
            }
          });
        }}>Cancel</div>
      </div>
    </div> as HTMLDivElement;

    container.appendChild(this._popup);
  }

  public show( text: HTMLElement, cb: () => void ){
    this._callback = cb;

    this._textContainer.innerHTML = '';
    this._textContainer.appendChild(text);

    this._popup.style.display = 'block';

    anime({
      targets: this._popup,
      opacity: [ 0, 1 ],
      easing: 'easeInOutQuad',
      duration: 100
    });
  }

  public static Init( container: HTMLDivElement ){
    window.ConfirmationManager = new ConfirmationManager(container); // When "Init" is called create a new instance of ConfirmationManager and fill in "window.ConfirmationManager"
  }
}

export { ConfirmationManager } // Export ConfirmationManager so other scripts can use the type / call static functions