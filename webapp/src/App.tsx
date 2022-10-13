import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import styled, { ThemeProvider, createGlobalStyle } from "styled-components";

import { DarkTheme } from "./theme/theme";
import { LoginRoute } from "./routes/login/Login";
import { MainRoute } from "./routes/main/Main";

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
        <BrowserRouter>
          <Routes>
            <Route index element={<MainRoute />} />
            <Route path="/login" element={<LoginRoute />} />
          </Routes>
        </BrowserRouter>
      </Container>
      <GlobalStyle />
    </ThemeProvider>
  );
}

export default App;
