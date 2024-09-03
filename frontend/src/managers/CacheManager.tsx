import * as cooki from './CookiManager';

// Add an element to the global "window" variable to store the cache
declare global{
  interface Window{
    CacheManager: CacheManager
  }
}

class CacheManager{
  private cache: any = {};

  private constructor(){}

  public get( url: string ): Promise<any> {
    return new Promise<any>(( res, rej ) => {
      let cache = this.cache[`GET ${url}`];
      if(cache)return res(cache);

      fetch(url, { headers: { Authorization: `Bearer ${cooki.getStore('token')}` } })
        .then(data => data.json())
        .then(data => {
          this.cache[`GET ${url}`] = data;
          res(data);
        })
        .catch(rej);
    })
  }

  public static Init(){
    window.CacheManager = new CacheManager(); // When "Init" is called create a new instance of CacheManager and fill in "window.CacheManager"
  }
}

export { CacheManager } // Export CacheManager so other scripts can use the type / call static functions