import {
  BrowserRouter,
  Navigate,
  Route,
  Routes,
  useNavigate,
} from "react-router-dom";
import styled, { ThemeProvider, createGlobalStyle } from "styled-components";

import { Client } from "./lib/tenso";
import { DarkTheme } from "./theme/theme";
import { InitRoute } from "./routes/init/Init";
import { LoginRoute } from "./routes/login/Login";
import { MainRoute } from "./routes/main/Main";
import { Router } from "./Router";
import { useApi } from "./hooks/useApi";
import { useEffect } from "react";
import { useEffectAsync } from "./hooks/useEffectAsync";

const GlobalStyle = createGlobalStyle`
  body {
    font-family: 'Roboto', sans-serif;
    background-color: ${(p) => p.theme.background};
    padding: 0;
    margin: 0;
  }
  * {
    box-sizing: border-box;
  }
  h1, h2, h3, h4, h5, h6 {
    margin-top: 0;
  }
`;

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  color: ${(p) => p.theme.text};
`;

function App() {
  return (
    <ThemeProvider theme={DarkTheme}>
      <Container>
        <BrowserRouter basename="/ui">
          <Router />
        </BrowserRouter>
      </Container>
      <GlobalStyle />
    </ThemeProvider>
  );
}

export default App;
