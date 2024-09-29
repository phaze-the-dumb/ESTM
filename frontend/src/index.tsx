import { lazy } from "solid-js";
import { render } from 'solid-js/web';
import { Router, Route } from "@solidjs/router";
import { CacheManager } from "./managers/CacheManager";
import './index.css'

// Create the global "endpoint" constant
declare global{
  interface Window{
    ENDPOINT: string;
  }
}

window.ENDPOINT = ''; // Set the endpoint value to the backend url ( Leave blank in prod builds so it uses the base url of the site)

// Setup the CacheManager instance
CacheManager.Init();

// Import the main app template
import App from './App';

// Import each page of the app ( use "lazy" so they are only loaded when needed )
const Login = lazy(() => import('./pages/Login'));

const Dash = lazy(() => import('./pages/Dash'));
const DashMatches = lazy(() => import('./pages/Dash/Matches'));
const DashTeams = lazy(() => import('./pages/Dash/Teams'));
const DashBrackets = lazy(() => import('./pages/Dash/Brackets'));

// Call SolidJS to render the app to the user
render(() => (
  <Router root={App}>
    <Route path="/" component={Login} />

    <Route path="/dash" component={Dash} />
    <Route path="/dash/matches" component={DashMatches} />
    <Route path="/dash/teams" component={DashTeams} />
    <Route path="/dash/brackets" component={DashBrackets} />
  </Router>
), document.getElementById('root')!);

let font = new FontFace('Rubik', '/Rubik-VariableFont_wght.ttf');
document.fonts.add(font);